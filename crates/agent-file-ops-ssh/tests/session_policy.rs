#[test]
fn session_source_validates_configuration_before_connecting() {
    let source = include_str!("../src/session.rs");
    assert!(source.contains(".validate()"));
    assert!(source.contains("TransportError::InvalidConfig"));
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
fn session_source_does_not_open_a_shell() {
    let source = include_str!("../src/session.rs");
    assert!(!source.contains("request_shell("));
}
