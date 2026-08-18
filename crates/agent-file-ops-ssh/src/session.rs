use crate::{
    AgentFileOpsSftp, CredentialRef, SshTransportConfig, StrictHostKeyVerifier, TransportError,
};
use russh::client;
use russh::keys::agent::{client::AgentClient, AgentIdentity};
use russh::keys::{load_secret_key, PrivateKeyWithHashAlg, PublicKey};
use russh_sftp::client::SftpSession;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
enum SessionHandlerError {
    #[error(transparent)]
    Russh(#[from] russh::Error),

    #[error(transparent)]
    HostKey(#[from] TransportError),
}

struct StrictClientHandler {
    verifier: StrictHostKeyVerifier,
}

impl client::Handler for StrictClientHandler {
    type Error = SessionHandlerError;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        self.verifier.verify(server_public_key)?;
        Ok(true)
    }
}

pub struct AgentFileOpsSshSession {
    handle: client::Handle<StrictClientHandler>,
    operation_timeout: std::time::Duration,
    inline_read_bytes: u64,
}

impl AgentFileOpsSshSession {
    pub async fn connect(config: SshTransportConfig) -> Result<Self, TransportError> {
        let verifier = StrictHostKeyVerifier::new(
            config.host.clone(),
            config.port,
            config.known_hosts_path.clone(),
        )?;

        let client_config = Arc::new(client::Config {
            inactivity_timeout: Some(config.operation_timeout),
            ..Default::default()
        });

        let address = (config.host.as_str(), config.port);
        let connect_result = tokio::time::timeout(
            config.connect_timeout,
            client::connect(
                client_config,
                address,
                StrictClientHandler { verifier },
            ),
        )
        .await
        .map_err(|_| TransportError::ConnectionTimeout)?;

        let mut handle = match connect_result {
            Ok(handle) => handle,
            Err(SessionHandlerError::HostKey(error)) => return Err(error),
            Err(SessionHandlerError::Russh(error)) => return Err(map_russh_error(error)),
        };

        authenticate(&mut handle, &config).await?;

        Ok(Self {
            handle,
            operation_timeout: config.operation_timeout,
            inline_read_bytes: config.inline_read_bytes,
        })
    }

    pub fn is_closed(&self) -> bool {
        self.handle.is_closed()
    }

    pub async fn open_sftp(&self) -> Result<AgentFileOpsSftp, TransportError> {
        let channel = tokio::time::timeout(
            self.operation_timeout,
            self.handle.channel_open_session(),
        )
        .await
        .map_err(|_| TransportError::ConnectionTimeout)?
        .map_err(map_russh_error)?;

        tokio::time::timeout(
            self.operation_timeout,
            channel.request_subsystem(true, "sftp"),
        )
        .await
        .map_err(|_| TransportError::ConnectionTimeout)?
        .map_err(map_russh_error)?;

        let sftp = SftpSession::new(channel.into_stream())
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        sftp.set_timeout(self.operation_timeout.as_secs().max(1));
        Ok(AgentFileOpsSftp::new(sftp, self.inline_read_bytes))
    }

    pub async fn close(self) -> Result<(), TransportError> {
        self.handle
            .disconnect(russh::Disconnect::ByApplication, "", "")
            .await
            .map_err(map_russh_error)
    }
}

async fn authenticate(
    handle: &mut client::Handle<StrictClientHandler>,
    config: &SshTransportConfig,
) -> Result<(), TransportError> {
    match &config.credential_ref {
        CredentialRef::KeyFile(path) => {
            let private_key = load_secret_key(path, None)
                .map_err(|error| TransportError::KeyLoadFailed(error.to_string()))?;
            let hash_alg = handle
                .best_supported_rsa_hash()
                .await
                .map_err(map_russh_error)?
                .flatten();
            let key = PrivateKeyWithHashAlg::new(Arc::new(private_key), hash_alg);
            let result = handle
                .authenticate_publickey(config.username.clone(), key)
                .await
                .map_err(map_russh_error)?;
            if result.success() {
                Ok(())
            } else {
                Err(TransportError::AuthenticationFailed)
            }
        }
        CredentialRef::SshAgent => authenticate_with_agent(handle, &config.username).await,
    }
}

#[cfg(unix)]
async fn authenticate_with_agent(
    handle: &mut client::Handle<StrictClientHandler>,
    username: &str,
) -> Result<(), TransportError> {
    let mut agent = AgentClient::connect_env()
        .await
        .map_err(|error| TransportError::AgentUnavailable(error.to_string()))?;
    let identities = agent
        .request_identities()
        .await
        .map_err(|error| TransportError::AgentUnavailable(error.to_string()))?;

    for identity in identities {
        let hash_alg = handle
            .best_supported_rsa_hash()
            .await
            .map_err(map_russh_error)?
            .flatten();

        let authenticated = match identity {
            AgentIdentity::PublicKey { key, .. } => handle
                .authenticate_publickey_with(username.to_string(), key, hash_alg, &mut agent)
                .await
                .map_err(|error| TransportError::AgentUnavailable(error.to_string()))?
                .success(),
            AgentIdentity::Certificate { certificate, .. } => handle
                .authenticate_certificate_with(
                    username.to_string(),
                    certificate,
                    hash_alg,
                    &mut agent,
                )
                .await
                .map_err(|error| TransportError::AgentUnavailable(error.to_string()))?
                .success(),
        };

        if authenticated {
            return Ok(());
        }
    }

    Err(TransportError::AuthenticationFailed)
}

#[cfg(not(unix))]
async fn authenticate_with_agent(
    _handle: &mut client::Handle<StrictClientHandler>,
    _username: &str,
) -> Result<(), TransportError> {
    Err(TransportError::AgentUnavailable(
        "SSH-agent authentication is not yet implemented for this platform".into(),
    ))
}

fn map_russh_error(error: russh::Error) -> TransportError {
    match error {
        russh::Error::ConnectionTimeout | russh::Error::Elapsed(_) => {
            TransportError::ConnectionTimeout
        }
        russh::Error::NotAuthenticated | russh::Error::NoAuthMethod => {
            TransportError::AuthenticationFailed
        }
        other => TransportError::ConnectionFailed(other.to_string()),
    }
}
