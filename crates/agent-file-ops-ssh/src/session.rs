use crate::{CredentialRef, SshTransportConfig, TransportError};
use std::sync::Arc;

/// AgentFileOps SSH session lifecycle manager.
///
/// Manages authentication, known-hosts verification, and session state.
/// Credentials are never stored directly; only references are kept.
pub struct AgentFileOpsSshSession {
    config: Arc<SshTransportConfig>,
    authenticated: bool,
}

impl AgentFileOpsSshSession {
    /// Create a new SSH session (not yet connected).
    pub fn new(config: SshTransportConfig) -> Self {
        Self {
            config: Arc::new(config),
            authenticated: false,
        }
    }

    /// Get the configuration (non-mutable access).
    pub fn config(&self) -> &SshTransportConfig {
        &self.config
    }

    /// Check if session is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    /// Connect and authenticate using the credential reference.
    ///
    /// In a real implementation, this would:
    /// 1. Resolve credential_ref from a secure store
    /// 2. Connect to host:port
    /// 3. Verify host key against known_hosts_ref
    /// 4. Authenticate with the resolved credential
    ///
    /// For now, this is a placeholder that validates configuration.
    pub async fn connect(&mut self) -> Result<(), TransportError> {
        // Validate configuration before attempting connection
        self.config.validate().map_err(TransportError::InvalidConfig)?;

        // In a real implementation:
        // - Resolve self.config.known_hosts_ref
        // - Resolve self.config.credential_ref
        // - Create SSH connection
        // - Perform strict host key verification
        // - Authenticate
        // - Mark authenticated = true

        self.authenticated = true;
        Ok(())
    }

    /// Disconnect and clean up session.
    pub async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.authenticated = false;
        Ok(())
    }

    /// Perform a credential reference lookup (mock for now).
    ///
    /// Real implementation would resolve env vars, vault paths, etc.
    pub fn resolve_credential_ref(&self, _cred_ref: &CredentialRef) -> Result<String, TransportError> {
        // Placeholder: in production, resolve from secure store
        Err(TransportError::AgentUnavailable(
            "credential resolution not yet implemented".to_string(),
        ))
    }
}

impl Drop for AgentFileOpsSshSession {
    fn drop(&mut self) {
        // Cleanup: close connection if open
        self.authenticated = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_unconnected_session() {
        let config = SshTransportConfig::new("example.com", 22, "known_hosts", "my-key");
        let session = AgentFileOpsSshSession::new(config);

        assert!(!session.is_authenticated());
    }

    #[tokio::test]
    async fn validates_config_on_connect() {
        let config = SshTransportConfig::new("example.com", 22, "known_hosts", "my-key");
        let mut session = AgentFileOpsSshSession::new(config);

        // Should succeed (config is valid)
        assert!(session.connect().await.is_ok());
        assert!(session.is_authenticated());
    }
}
