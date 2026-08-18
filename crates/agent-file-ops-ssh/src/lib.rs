mod config;
mod credentials;
mod host_key;
mod session;
mod sftp_ops;

pub use config::{SshTransportConfig, MAX_INLINE_READ_BYTES};
pub use credentials::CredentialRef;
pub use host_key::StrictHostKeyVerifier;
pub use session::AgentFileOpsSshSession;
pub use sftp_ops::{AgentFileOpsSftp, RemoteEntry, RemoteFileType, RemoteStat, WriteResult};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("invalid SSH transport configuration: {0}")]
    InvalidConfig(String),

    #[error("known_hosts file is unavailable: {0}")]
    KnownHostsUnavailable(String),

    #[error("SSH host key is unknown for {host}:{port}")]
    UnknownHostKey { host: String, port: u16 },

    #[error("SSH host key mismatch for {host}:{port} at known_hosts line {line}")]
    HostKeyMismatch {
        host: String,
        port: u16,
        line: usize,
    },

    #[error("SSH host key verification failed for {host}:{port}: {message}")]
    HostKeyVerificationFailed {
        host: String,
        port: u16,
        message: String,
    },

    #[error("SSH agent is unavailable: {0}")]
    AgentUnavailable(String),

    #[error("SSH key could not be loaded: {0}")]
    KeyLoadFailed(String),

    #[error("SSH authentication failed")]
    AuthenticationFailed,

    #[error("SSH connection timed out")]
    ConnectionTimeout,

    #[error("SSH connection failed: {0}")]
    ConnectionFailed(String),

    #[error("SFTP operation failed: {0}")]
    Sftp(String),

    #[error("read exceeded configured limit of {limit} bytes")]
    ReadLimitExceeded { limit: u64 },

    #[error("remote destination already exists: {0}")]
    Conflict(String),
}
