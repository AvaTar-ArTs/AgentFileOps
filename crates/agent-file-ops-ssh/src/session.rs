use crate::{
    AgentFileOpsSftp, CredentialRef, SshTransportConfig, StrictHostKeyVerifier, TransportError,
};
use russh::client::{self, Handle};
use russh::keys::{load_secret_key, PrivateKeyWithHashAlg};
use russh::Disconnect;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
enum SessionError {
    #[error(transparent)]
    Russh(#[from] russh::Error),
    #[error(transparent)]
    Transport(#[from] TransportError),
}

struct Handler {
    host: String,
    port: u16,
    known_hosts: String,
}

impl client::Handler for Handler {
    type Error = SessionError;

    async fn check_server_key(
        &mut self,
        server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        StrictHostKeyVerifier::verify_public_key(
            &self.host,
            self.port,
            server_public_key,
            &self.known_hosts,
        )?;
        Ok(true)
    }
}

/// Live SSH session with strict host-key verification and SFTP-only public operations.
pub struct AgentFileOpsSshSession {
    config: Arc<SshTransportConfig>,
    handle: Option<Handle<Handler>>,
}

impl AgentFileOpsSshSession {
    /// Create a disconnected session configuration.
    pub fn new(config: SshTransportConfig) -> Self {
        Self {
            config: Arc::new(config),
            handle: None,
        }
    }

    pub fn config(&self) -> &SshTransportConfig {
        &self.config
    }

    pub fn is_authenticated(&self) -> bool {
        self.handle.is_some()
    }

    /// Connect, verify the server key, and authenticate with a referenced key file.
    pub async fn connect(&mut self) -> Result<(), TransportError> {
        self.config
            .validate()
            .map_err(TransportError::InvalidConfig)?;
        let username =
            self.config.username.as_deref().ok_or_else(|| {
                TransportError::InvalidConfig("username must be configured".into())
            })?;
        let key_path = self.resolve_credential_path()?;
        let key = load_secret_key(&key_path, None)
            .map_err(|error| TransportError::KeyLoadFailed(error.to_string()))?;
        let handler = Handler {
            host: self.config.host.clone(),
            port: self.config.port,
            known_hosts: self.config.known_hosts_ref.clone(),
        };
        let ssh_config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(self.config.operation_timeout_secs)),
            ..Default::default()
        };
        let mut handle = tokio::time::timeout(
            Duration::from_secs(self.config.connect_timeout_secs),
            client::connect(
                Arc::new(ssh_config),
                (self.config.host.as_str(), self.config.port),
                handler,
            ),
        )
        .await
        .map_err(|_| TransportError::ConnectionTimeout)?
        .map_err(session_error)?;

        let auth = handle
            .authenticate_publickey(
                username,
                PrivateKeyWithHashAlg::new(
                    Arc::new(key),
                    handle
                        .best_supported_rsa_hash()
                        .await
                        .map_err(|error| session_error(SessionError::Russh(error)))?
                        .flatten(),
                ),
            )
            .await
            .map_err(|error| session_error(SessionError::Russh(error)))?;
        if !auth.success() {
            return Err(TransportError::AuthenticationFailed);
        }
        self.handle = Some(handle);
        Ok(())
    }

    /// Open the SFTP subsystem. No shell or generic command channel is exposed.
    pub async fn open_sftp(&mut self) -> Result<AgentFileOpsSftp, TransportError> {
        let handle = self
            .handle
            .as_mut()
            .ok_or_else(|| TransportError::ConnectionFailed("session is not connected".into()))?;
        let channel = handle
            .channel_open_session()
            .await
            .map_err(|error| TransportError::ConnectionFailed(error.to_string()))?;
        channel
            .request_subsystem(true, "sftp")
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        let sftp = russh_sftp::client::SftpSession::new(channel.into_stream())
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        sftp.set_timeout(self.config.operation_timeout_secs);
        Ok(AgentFileOpsSftp::from_session(
            sftp,
            self.config.inline_read_limit.get(),
        ))
    }

    pub async fn disconnect(&mut self) -> Result<(), TransportError> {
        if let Some(handle) = self.handle.take() {
            handle
                .disconnect(Disconnect::ByApplication, "", "AgentFileOps")
                .await
                .map_err(|error| TransportError::ConnectionFailed(error.to_string()))?;
        }
        Ok(())
    }

    fn resolve_credential_path(&self) -> Result<String, TransportError> {
        let reference = CredentialRef::parse(&self.config.credential_ref);
        let path = if let Some(name) = reference.env_var_name() {
            std::env::var(name).map_err(|_| {
                TransportError::KeyLoadFailed(format!(
                    "credential path environment variable is unavailable: {name}"
                ))
            })?
        } else {
            reference.ref_id
        };
        if path.is_empty() {
            return Err(TransportError::KeyLoadFailed(
                "credential path is empty".into(),
            ));
        }
        Ok(path)
    }
}

fn session_error(error: SessionError) -> TransportError {
    match error {
        SessionError::Transport(error) => error,
        SessionError::Russh(error) => TransportError::ConnectionFailed(error.to_string()),
    }
}

impl Drop for AgentFileOpsSshSession {
    fn drop(&mut self) {
        self.handle.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_unconnected_session() {
        let config = SshTransportConfig::new("example.com", 22, "known_hosts", "my-key")
            .with_username("agent");
        let session = AgentFileOpsSshSession::new(config);
        assert!(!session.is_authenticated());
    }

    #[tokio::test]
    async fn refuses_live_connect_without_username() {
        let config = SshTransportConfig::new("example.com", 22, "known_hosts", "my-key");
        let mut session = AgentFileOpsSshSession::new(config);
        assert!(matches!(
            session.connect().await,
            Err(TransportError::InvalidConfig(message)) if message == "username must be configured"
        ));
    }
}
