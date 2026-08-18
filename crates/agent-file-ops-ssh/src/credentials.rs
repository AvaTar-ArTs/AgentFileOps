use serde::Serialize;
use std::path::PathBuf;

/// A reference to credential material owned by the runtime environment.
///
/// Secret bytes are never stored in this enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CredentialRef {
    /// Use the runtime user's SSH agent.
    SshAgent,

    /// Use a private key mounted or otherwise supplied as a local secret file.
    KeyFile(PathBuf),
}
