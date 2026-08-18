use crate::TransportError;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::{FileAttributes, OpenFlags};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteStat {
    pub size: u64,
    pub permissions: u32,
    pub mtime: u64,
    pub file_type: RemoteFileType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteFileType {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteEntry {
    pub name: String,
    pub stat: RemoteStat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    pub bytes_written: u64,
    pub checksum: Option<String>,
}

/// Semantic SFTP operations backed by a live russh SFTP subsystem.
pub struct AgentFileOpsSftp {
    session: Arc<SftpSession>,
    inline_read_limit: u64,
}

impl AgentFileOpsSftp {
    pub(crate) fn from_session(session: SftpSession, inline_read_limit: u64) -> Self {
        Self {
            session: Arc::new(session),
            inline_read_limit,
        }
    }

    pub async fn list(
        &self,
        path: &str,
        limit: Option<usize>,
    ) -> Result<Vec<RemoteEntry>, TransportError> {
        let mut entries = Vec::new();
        for entry in self.session.read_dir(path).await.map_err(sftp_error)? {
            if limit.is_some_and(|max| entries.len() >= max) {
                break;
            }
            let metadata = entry.metadata();
            entries.push(RemoteEntry {
                name: entry.file_name(),
                stat: metadata.into(),
            });
        }
        Ok(entries)
    }

    pub async fn lstat(&self, path: &str) -> Result<RemoteStat, TransportError> {
        self.session
            .symlink_metadata(path)
            .await
            .map(Into::into)
            .map_err(sftp_error)
    }

    pub async fn stat(&self, path: &str) -> Result<RemoteStat, TransportError> {
        self.session
            .metadata(path)
            .await
            .map(Into::into)
            .map_err(sftp_error)
    }

    pub async fn read(
        &self,
        path: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<u8>, TransportError> {
        if limit == 0 || limit > self.inline_read_limit {
            return Err(TransportError::ReadLimitExceeded {
                limit: self.inline_read_limit,
            });
        }
        let mut file = self.session.open(path).await.map_err(sftp_error)?;
        file.seek(std::io::SeekFrom::Start(offset))
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        let mut data = Vec::with_capacity(limit as usize);
        file.take(limit + 1)
            .read_to_end(&mut data)
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        if data.len() as u64 > limit {
            return Err(TransportError::ReadLimitExceeded { limit });
        }
        Ok(data)
    }

    pub async fn write_new(&self, path: &str, data: &[u8]) -> Result<WriteResult, TransportError> {
        if self.session.try_exists(path).await.map_err(sftp_error)? {
            return Err(TransportError::Conflict(path.to_string()));
        }
        let mut file = self
            .session
            .open_with_flags(
                path,
                OpenFlags::CREATE | OpenFlags::EXCLUDE | OpenFlags::WRITE,
            )
            .await
            .map_err(sftp_error)?;
        file.write_all(data)
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        file.shutdown()
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        Ok(WriteResult {
            bytes_written: data.len() as u64,
            checksum: None,
        })
    }

    pub async fn mkdir(&self, path: &str, _mode: u32) -> Result<(), TransportError> {
        self.session.create_dir(path).await.map_err(sftp_error)
    }
}

fn sftp_error(error: impl std::fmt::Display) -> TransportError {
    TransportError::Sftp(error.to_string())
}

impl From<FileAttributes> for RemoteStat {
    fn from(attributes: FileAttributes) -> Self {
        let file_type = attributes.file_type();
        let file_type = if file_type.is_file() {
            RemoteFileType::File
        } else if file_type.is_dir() {
            RemoteFileType::Directory
        } else if file_type.is_symlink() {
            RemoteFileType::Symlink
        } else {
            RemoteFileType::Other
        };
        Self {
            size: attributes.size.unwrap_or_default(),
            permissions: attributes.permissions.unwrap_or_default(),
            mtime: attributes.mtime.unwrap_or_default() as u64,
            file_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_conversion_preserves_symlink_identity() {
        let mut attributes = FileAttributes::empty();
        attributes.size = Some(12);
        attributes.permissions = Some(0o120777);
        let stat: RemoteStat = attributes.into();
        assert_eq!(stat.size, 12);
        assert_eq!(stat.file_type, RemoteFileType::Symlink);
    }

    #[test]
    fn metadata_conversion_defaults_missing_fields_safely() {
        let stat: RemoteStat = FileAttributes::empty().into();
        assert_eq!(stat.size, 0);
        assert_eq!(stat.permissions, 0);
        assert_eq!(stat.mtime, 0);
        assert_eq!(stat.file_type, RemoteFileType::Other);
    }
}
