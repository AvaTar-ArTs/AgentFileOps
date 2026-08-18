use agent_file_ops_ssh::{
    CredentialRef, SshTransportConfig, StrictHostKeyVerifier, TransportError,
    MAX_INLINE_READ_BYTES,
};

fn base_config(read_limit: u64) -> Result<SshTransportConfig, TransportError> {
    if read_limit == 0 {
        return Err(TransportError::InvalidConfig(
            "inline_read_limit must be non-zero".to_string(),
        ));
    }

    Ok(SshTransportConfig::new(
        "example.test",
        22,
        "agentfileops",
        "env:SSH_PRIVATE_KEY",
    )
    .with_inline_read_limit(read_limit))
}

#[test]
fn rejects_zero_inline_read_limit() {
    assert!(matches!(
        base_config(0),
        Err(TransportError::InvalidConfig(_))
    ));
}

#[test]
fn rejects_inline_read_limit_above_contract_maximum() {
    let config = base_config(MAX_INLINE_READ_BYTES + 1).expect("config is clamped");
    assert_eq!(config.inline_read_limit.get(), MAX_INLINE_READ_BYTES);
}

#[test]
fn accepts_contract_maximum_inline_read_limit() {
    let config = base_config(MAX_INLINE_READ_BYTES).expect("valid max read limit");
    assert_eq!(config.inline_read_limit.get(), MAX_INLINE_READ_BYTES);
}

#[test]
fn credential_reference_contains_no_raw_secret_value() {
    let env_ref = CredentialRef::parse("env:SSH_PRIVATE_KEY");
    let vault_ref = CredentialRef::parse("vault:prod/ssh-key");

    assert!(env_ref.is_env_ref());
    assert_eq!(env_ref.env_var_name(), Some("SSH_PRIVATE_KEY"));
    assert!(vault_ref.is_vault_ref());
    assert_eq!(vault_ref.vault_path(), Some("prod/ssh-key"));
}

#[test]
fn missing_known_hosts_file_fails_closed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let missing = dir.path().join("does-not-exist");

    let result = StrictHostKeyVerifier::verify(
        "example.test",
        22,
        "ssh-ed25519",
        "AAAA...",
        missing,
    );
    assert!(matches!(
        result,
        Err(TransportError::KnownHostsUnavailable(_))
    ));
}

#[test]
fn existing_known_hosts_file_initializes_verifier() {
    let dir = tempfile::tempdir().expect("tempdir");
    let known_hosts = dir.path().join("known_hosts");
    std::fs::write(&known_hosts, "").expect("write known_hosts");

    let result = StrictHostKeyVerifier::verify(
        "example.test",
        2222,
        "ssh-ed25519",
        "AAAA...",
        known_hosts,
    );
    assert!(matches!(result, Err(TransportError::UnknownHostKey { .. })));
}
