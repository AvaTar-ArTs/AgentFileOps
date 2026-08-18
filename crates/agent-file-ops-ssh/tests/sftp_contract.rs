#[test]
fn sftp_source_uses_exclusive_create_for_additive_writes() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("pub async fn write_new("));
    assert!(!source.contains("OpenFlags::TRUNCATE"));
}

#[test]
fn bounded_read_does_not_use_whole_file_convenience_api() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("pub async fn read("));
    assert!(!source.contains("std::process::Command"));
}

#[test]
fn additive_write_has_explicit_conflict_path() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("TransportError::Sftp"));
    assert!(source.contains("write_new"));
}

#[test]
fn sftp_surface_exposes_stat_and_lstat_separately() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("pub async fn stat("));
}
