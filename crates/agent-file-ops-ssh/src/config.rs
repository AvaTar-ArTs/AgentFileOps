use std::num::NonZeroU64;

pub const MAX_INLINE_READ_BYTES: u64 = 16_777_216; // 16 MiB
pub const DEFAULT_INLINE_READ_BYTES: u64 = 1_048_576; // 1 MiB
pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 20;
pub const DEFAULT_OPERATION_TIMEOUT_SECS: u64 = 60;

/// SSH transport configuration with security constraints.
/// No raw credentials or host keys are embedded; only references are allowed.
#[derive(Debug, Clone)]
pub struct SshTransportConfig {
    /// Hostname or IP address
    pub host: String,

    /// SSH port (default 22)
    pub port: u16,

    /// Username for SSH authentication
    pub username: Option<String>,

    /// Reference to known_hosts store (no embedded keys)
    pub known_hosts_ref: String,

    /// Reference to credential store (no embedded secrets)
    pub credential_ref: String,

    /// Maximum bytes for inline reads (default 1 MiB, max 16 MiB)
    pub inline_read_limit: NonZeroU64,

    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,

    /// Operation timeout in seconds
    pub operation_timeout_secs: u64,
}

impl SshTransportConfig {
    /// Create a new SSH transport configuration.
    ///
    /// # Arguments
    /// * `host` - Hostname or IP address
    /// * `port` - SSH port (1-65535)
    /// * `known_hosts_ref` - Reference to known_hosts material
    /// * `credential_ref` - Reference to SSH credentials
    ///
    /// # Panics
    /// If port is 0 or greater than 65535.
    pub fn new(
        host: impl Into<String>,
        port: u16,
        known_hosts_ref: impl Into<String>,
        credential_ref: impl Into<String>,
    ) -> Self {
        assert!(port > 0, "SSH port must be 1-65535");

        Self {
            host: host.into(),
            port,
            username: None,
            known_hosts_ref: known_hosts_ref.into(),
            credential_ref: credential_ref.into(),
            inline_read_limit: NonZeroU64::new(DEFAULT_INLINE_READ_BYTES)
                .expect("default read limit is non-zero"),
            connect_timeout_secs: DEFAULT_CONNECT_TIMEOUT_SECS,
            operation_timeout_secs: DEFAULT_OPERATION_TIMEOUT_SECS,
        }
    }

    /// Set the username for authentication.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Set inline read limit (will be clamped to max).
    pub fn with_inline_read_limit(mut self, limit: u64) -> Self {
        self.inline_read_limit = NonZeroU64::new(limit.min(MAX_INLINE_READ_BYTES))
            .expect("inline read limit is non-zero");
        self
    }

    /// Validate configuration constraints.
    pub fn validate(&self) -> Result<(), String> {
        if self.host.is_empty() {
            return Err("host must not be empty".to_string());
        }

        if self.known_hosts_ref.is_empty() {
            return Err("known_hosts_ref must not be empty".to_string());
        }

        if self.credential_ref.is_empty() {
            return Err("credential_ref must not be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_config() {
        let config = SshTransportConfig::new(
            "example.com",
            22,
            "path/to/known_hosts",
            "my-ssh-key",
        );
        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, 22);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn clamps_inline_read_limit() {
        let config = SshTransportConfig::new(
            "example.com",
            22,
            "path/to/known_hosts",
            "my-ssh-key",
        )
        .with_inline_read_limit(100_000_000_000); // way too large

        assert_eq!(config.inline_read_limit.get(), MAX_INLINE_READ_BYTES);
    }
}
