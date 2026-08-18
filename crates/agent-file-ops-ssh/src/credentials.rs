/// Reference to SSH credentials stored externally.
///
/// The protocol forbids embedding raw credentials (private keys, passwords, passphrases).
/// Implementations must resolve these references at runtime from a secure store.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CredentialRef {
    /// Reference identifier (e.g., "my-ssh-key", "env:SSH_KEY", "vault:prod/ssh")
    pub ref_id: String,
}

impl CredentialRef {
    /// Create a new credential reference.
    pub fn new(ref_id: impl Into<String>) -> Self {
        Self {
            ref_id: ref_id.into(),
        }
    }

    /// Parse a credential reference from a string.
    /// Supports formats like:
    /// - "my-key" (local reference)
    /// - "env:SSH_PRIVATE_KEY" (environment variable)
    /// - "vault:path/to/secret" (external secret store)
    pub fn parse(input: &str) -> Self {
        Self {
            ref_id: input.to_string(),
        }
    }

    /// Check if this is an environment variable reference.
    pub fn is_env_ref(&self) -> bool {
        self.ref_id.starts_with("env:")
    }

    /// Extract environment variable name if applicable.
    pub fn env_var_name(&self) -> Option<&str> {
        self.ref_id.strip_prefix("env:")
    }

    /// Check if this is a vault reference.
    pub fn is_vault_ref(&self) -> bool {
        self.ref_id.starts_with("vault:")
    }

    /// Extract vault path if applicable.
    pub fn vault_path(&self) -> Option<&str> {
        self.ref_id.strip_prefix("vault:")
    }
}

impl std::fmt::Display for CredentialRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ref_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_env_reference() {
        let cred = CredentialRef::parse("env:SSH_PRIVATE_KEY");
        assert!(cred.is_env_ref());
        assert_eq!(cred.env_var_name(), Some("SSH_PRIVATE_KEY"));
    }

    #[test]
    fn parses_vault_reference() {
        let cred = CredentialRef::parse("vault:prod/ssh-key");
        assert!(cred.is_vault_ref());
        assert_eq!(cred.vault_path(), Some("prod/ssh-key"));
    }

    #[test]
    fn parses_simple_reference() {
        let cred = CredentialRef::parse("my-key");
        assert!(!cred.is_env_ref());
        assert!(!cred.is_vault_ref());
        assert_eq!(cred.ref_id, "my-key");
    }
}
