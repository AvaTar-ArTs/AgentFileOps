use crate::{CredentialRef, TransportError};
use std::path::PathBuf;
use std::time::Duration;

pub const MAX_INLINE_READ_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshTransportConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub known_hosts_path: PathBuf,
    pub credential_ref: CredentialRef,
    pub connect_timeout: Duration,
    pub operation_timeout: Duration,
    pub inline_read_bytes: u64,
}

impl SshTransportConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        known_hosts_path: PathBuf,
        credential_ref: CredentialRef,
        connect_timeout: Duration,
        operation_timeout: Duration,
        inline_read_bytes: u64,
    ) -> Result<Self, TransportError> {
        let host = host.into();
        let username = username.into();

        if host.trim().is_empty() {
            return Err(TransportError::InvalidConfig("host must not be empty".into()));
        }
        if username.trim().is_empty() {
            return Err(TransportError::InvalidConfig(
                "username must not be empty".into(),
            ));
        }
        if port == 0 {
            return Err(TransportError::InvalidConfig("port must be non-zero".into()));
        }
        if connect_timeout.is_zero() {
            return Err(TransportError::InvalidConfig(
                "connect timeout must be greater than zero".into(),
            ));
        }
        if operation_timeout.is_zero() {
            return Err(TransportError::InvalidConfig(
                "operation timeout must be greater than zero".into(),
            ));
        }
        if inline_read_bytes == 0 || inline_read_bytes > MAX_INLINE_READ_BYTES {
            return Err(TransportError::InvalidConfig(format!(
                "inline_read_bytes must be between 1 and {MAX_INLINE_READ_BYTES}"
            )));
        }

        Ok(Self {
            host,
            port,
            username,
            known_hosts_path,
            credential_ref,
            connect_timeout,
            operation_timeout,
            inline_read_bytes,
        })
    }
}
