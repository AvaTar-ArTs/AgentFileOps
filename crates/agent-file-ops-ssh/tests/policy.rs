use agent_file_ops_ssh::{
    CredentialRef, SshTransportConfig, StrictHostKeyVerifier, TransportError,
    MAX_INLINE_READ_BYTES,
};
use std::path::PathBuf;
use std::time::Duration;

fn base_config(read_limit: u64) -> Result<SshTransportConfig, TransportError> {
    SshTransportConfig::new(
        "example.test",
        22,
        "agentfileops",
        PathBuf::from("/tmp/known_hosts"),
        CredentialRef::SshAgent,
        Duration::from_secs(20),
        Duration::from_secs(60),
        read_limit,
    )
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
    assert!(matches!(
        base_config(MAX_INLINE_READ_BYTES + 1),
        Err(TransportError::InvalidConfig(_))
    ));
}

#[test]
fn accepts_contract_maximum_inline_read_limit() {
    let config = base_config(MAX_INLINE_READ_BYTES).expect("valid max read limit");
    assert_eq!(config.inline_read_bytes, MAX_INLINE_READ_BYTES);
}

#[test]
fn credential_reference_contains_no_raw_secret_value() {
    let agent = CredentialRef::SshAgent;
    let key = CredentialRef::KeyFile(PathBuf::from("/run/secrets/agentfileops_key"));

    assert!(matches!(agent, CredentialRef::SshAgent));
    assert!(matches!(key, CredentialRef::KeyFile(_)));
}

#[test]
fn missing_known_hosts_file_fails_closed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let missing = dir.path().join("does-not-exist");

    let result = StrictHostKeyVerifier::new("example.test", 22, missing);
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

    let verifier = StrictHostKeyVerifier::new("example.test", 2222, known_hosts.clone())
        .expect("verifier");
    assert_eq!(verifier.host(), "example.test");
    assert_eq!(verifier.port(), 2222);
    assert_eq!(verifier.known_hosts_path(), known_hosts.as_path());
}
