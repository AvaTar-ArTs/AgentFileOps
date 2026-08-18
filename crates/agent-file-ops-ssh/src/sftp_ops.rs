use crate::TransportError;
use serde::{Deserialize, Serialize};

/// Metadata about a remote file or directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteStat {
    /// File size in bytes
    pub size: u64,

    /// Unix permission bits (0o755, etc.)
    pub permissions: u32,

    /// UNIX timestamp of last modification
    pub mtime: u64,

    /// File type (file, directory, symlink, etc.)
    pub file_type: RemoteFileType,
}

/// Classifies remote file types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteFileType {
    /// Regular file
    File,

    /// Directory
    Directory,

    /// Symbolic link
    Symlink,

    /// Other type (device, socket, etc.)
    Other,
}

/// Entry from a remote directory listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteEntry {
    /// File or directory name (not full path)
    pub name: String,

    /// Metadata
    pub stat: RemoteStat,
}

/// Result of a remote file write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    /// Number of bytes written
    pub bytes_written: u64,

    /// SHA256 checksum of written data (optional)
    pub checksum: Option<String>,
}

/// Wrapper for SFTP operations over AgentFileOps protocol.
///
/// Enforces semantic operations instead of raw SFTP commands.
/// No arbitrary shell execution is exposed.
pub struct AgentFileOpsSftp {
    // In a real implementation, this would hold a live SFTP session
    _private: (),
}

impl AgentFileOpsSftp {
    /// Create a new SFTP session wrapper.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// List directory contents.
    ///
    /// # Arguments
    /// * `path` - Remote path to list
    /// * `limit` - Maximum entries to return
    pub async fn list(
        &self,
        _path: &str,
        _limit: Option<usize>,
    ) -> Result<Vec<RemoteEntry>, TransportError> {
        // Placeholder: real implementation would call SFTP opendir/readdir
        Err(TransportError::Sftp(
            "list operation not yet implemented".to_string(),
        ))
    }

    /// Get file metadata.
    pub async fn stat(&self, _path: &str) -> Result<RemoteStat, TransportError> {
        // Placeholder
        Err(TransportError::Sftp(
            "stat operation not yet implemented".to_string(),
        ))
    }

    /// Read file contents with byte limit.
    pub async fn read(
        &self,
        _path: &str,
        _offset: u64,
        _limit: u64,
    ) -> Result<Vec<u8>, TransportError> {
        // Placeholder
        Err(TransportError::Sftp(
            "read operation not yet implemented".to_string(),
        ))
    }

    /// Write file contents (new file only, no overwrite).
    pub async fn write_new(
        &self,
        _path: &str,
        _data: &[u8],
    ) -> Result<WriteResult, TransportError> {
        // Placeholder
        Err(TransportError::Sftp(
            "write_new operation not yet implemented".to_string(),
        ))
    }

    /// Create directory.
    pub async fn mkdir(&self, _path: &str, _mode: u32) -> Result<(), TransportError> {
        // Placeholder
        Err(TransportError::Sftp(
            "mkdir operation not yet implemented".to_string(),
        ))
    }

    /// Delete file.
    pub async fn delete(&self, _path: &str) -> Result<(), TransportError> {
        // Placeholder
        Err(TransportError::Sftp(
            "delete operation not yet implemented".to_string(),
        ))
    }
}

impl Default for AgentFileOpsSftp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_sftp_wrapper() {
        let _sftp = AgentFileOpsSftp::new();
        // Placeholder test
    }
}
