use crate::TransportError;
use russh::keys::known_hosts::check_known_hosts_path;
use russh::keys::PublicKey;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StrictHostKeyVerifier {
    host: String,
    port: u16,
    known_hosts_path: PathBuf,
}

impl StrictHostKeyVerifier {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        known_hosts_path: PathBuf,
    ) -> Result<Self, TransportError> {
        let host = host.into();
        if host.trim().is_empty() {
            return Err(TransportError::InvalidConfig("host must not be empty".into()));
        }
        if port == 0 {
            return Err(TransportError::InvalidConfig("port must be non-zero".into()));
        }
        if !known_hosts_path.is_file() {
            return Err(TransportError::KnownHostsUnavailable(
                known_hosts_path.display().to_string(),
            ));
        }

        Ok(Self {
            host,
            port,
            known_hosts_path,
        })
    }

    pub fn verify(&self, key: &PublicKey) -> Result<(), TransportError> {
        match check_known_hosts_path(&self.host, self.port, key, &self.known_hosts_path) {
            Ok(true) => Ok(()),
            Ok(false) => Err(TransportError::UnknownHostKey {
                host: self.host.clone(),
                port: self.port,
            }),
            Err(russh::keys::Error::KeyChanged { line }) => {
                Err(TransportError::HostKeyMismatch {
                    host: self.host.clone(),
                    port: self.port,
                    line,
                })
            }
            Err(error) => Err(TransportError::HostKeyVerificationFailed {
                host: self.host.clone(),
                port: self.port,
                message: error.to_string(),
            }),
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn known_hosts_path(&self) -> &std::path::Path {
        &self.known_hosts_path
    }
}
