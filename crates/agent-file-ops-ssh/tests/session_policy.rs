#[test]
fn session_source_has_strict_server_key_callback() {
    let source = include_str!("../src/session.rs");
    assert!(source.contains("check_server_key"));
    assert!(source.contains("StrictHostKeyVerifier"));
}

#[test]
fn session_source_exposes_no_generic_exec_api() {
    let source = include_str!("../src/session.rs");
    for forbidden in [
        "pub fn exec(",
        "pub async fn exec(",
        "pub fn run_shell(",
        "pub async fn run_shell(",
        "pub fn ssh_exec(",
        "pub async fn ssh_exec(",
    ] {
        assert!(!source.contains(forbidden), "forbidden public API: {forbidden}");
    }
}

#[test]
fn session_source_opens_sftp_as_a_subsystem_not_shell() {
    let source = include_str!("../src/session.rs");
    assert!(source.contains("request_subsystem(true, \"sftp\")"));
    assert!(!source.contains("request_shell("));
}
