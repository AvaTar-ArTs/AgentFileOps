use agent_file_ops_ssh::{RemoteFileType, RemoteStat};
use russh_sftp::protocol::FileAttributes;

#[test]
fn metadata_conversion_preserves_symlink_identity() {
    let mut attrs = FileAttributes::empty();
    attrs.size = Some(12);
    attrs.permissions = Some(0o120777);
    let stat = RemoteStat::from(attrs);

    assert_eq!(stat.size, Some(12));
    assert_eq!(stat.file_type, RemoteFileType::Symlink);
}

#[test]
fn sftp_source_uses_exclusive_create_for_additive_writes() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("OpenFlags::CREATE | OpenFlags::EXCLUDE | OpenFlags::WRITE"));
    assert!(!source.contains("OpenFlags::TRUNCATE"));
}

#[test]
fn bounded_read_does_not_use_whole_file_convenience_api() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("file.take(max_bytes + 1)"));
    assert!(!source.contains("self.session.read(path)"));
}

#[test]
fn additive_write_has_explicit_conflict_path() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("TransportError::Conflict"));
    assert!(source.contains("try_exists"));
}

#[test]
fn sftp_surface_exposes_stat_and_lstat_separately() {
    let source = include_str!("../src/sftp_ops.rs");
    assert!(source.contains("pub async fn stat("));
    assert!(source.contains("pub async fn lstat("));
    assert!(source.contains("symlink_metadata"));
}
