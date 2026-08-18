use crate::TransportError;
use russh_sftp::{
    client::SftpSession,
    protocol::{FileAttributes, FileType, OpenFlags},
};
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteFileType {
    File,
    Directory,
    Symlink,
    Other,
}

impl From<FileType> for RemoteFileType {
    fn from(value: FileType) -> Self {
        match value {
            FileType::File => Self::File,
            FileType::Dir => Self::Directory,
            FileType::Symlink => Self::Symlink,
            FileType::Other => Self::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteStat {
    pub size: Option<u64>,
    pub file_type: RemoteFileType,
    pub permissions: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub atime: Option<u32>,
    pub mtime: Option<u32>,
}

impl From<FileAttributes> for RemoteStat {
    fn from(value: FileAttributes) -> Self {
        Self {
            size: value.size,
            file_type: value.file_type().into(),
            permissions: value.permissions,
            uid: value.uid,
            gid: value.gid,
            atime: value.atime,
            mtime: value.mtime,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteEntry {
    pub name: String,
    pub path: String,
    pub stat: RemoteStat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteResult {
    pub path: String,
    pub bytes_written: u64,
}

pub struct AgentFileOpsSftp {
    session: SftpSession,
    inline_read_bytes: u64,
}

impl AgentFileOpsSftp {
    pub fn new(session: SftpSession, inline_read_bytes: u64) -> Self {
        Self {
            session,
            inline_read_bytes,
        }
    }

    pub async fn close(&self) -> Result<(), TransportError> {
        self.session
            .close()
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))
    }

    pub async fn list(&self, path: &str) -> Result<Vec<RemoteEntry>, TransportError> {
        let entries = self
            .session
            .read_dir(path)
            .await
            .map_err(map_sftp_error)?;

        Ok(entries
            .map(|entry| RemoteEntry {
                name: entry.file_name(),
                path: entry.path(),
                stat: entry.metadata().into(),
            })
            .collect())
    }

    /// Follow the final symlink and return metadata for its target.
    pub async fn stat(&self, path: &str) -> Result<RemoteStat, TransportError> {
        self.session
            .metadata(path)
            .await
            .map(RemoteStat::from)
            .map_err(map_sftp_error)
    }

    /// Return metadata for the link itself rather than following it.
    pub async fn lstat(&self, path: &str) -> Result<RemoteStat, TransportError> {
        self.session
            .symlink_metadata(path)
            .await
            .map(RemoteStat::from)
            .map_err(map_sftp_error)
    }

    pub async fn read_bounded(&self, path: &str) -> Result<Vec<u8>, TransportError> {
        self.read_bounded_with_limit(path, self.inline_read_bytes).await
    }

    pub async fn read_bounded_with_limit(
        &self,
        path: &str,
        max_bytes: u64,
    ) -> Result<Vec<u8>, TransportError> {
        if max_bytes == 0 {
            return Err(TransportError::InvalidConfig(
                "bounded read limit must be greater than zero".into(),
            ));
        }

        let metadata = self.session.metadata(path).await.map_err(map_sftp_error)?;
        if metadata.size.is_some_and(|size| size > max_bytes) {
            return Err(TransportError::ReadLimitExceeded { limit: max_bytes });
        }

        let file = self.session.open(path).await.map_err(map_sftp_error)?;
        let mut limited = file.take(max_bytes + 1);
        let mut bytes = Vec::with_capacity(
            metadata
                .size
                .unwrap_or(0)
                .min(max_bytes)
                .try_into()
                .unwrap_or(0),
        );
        limited
            .read_to_end(&mut bytes)
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;

        if bytes.len() as u64 > max_bytes {
            return Err(TransportError::ReadLimitExceeded { limit: max_bytes });
        }

        Ok(bytes)
    }

    /// Create a new remote file atomically. Existing destinations are never overwritten.
    pub async fn write_new<R>(
        &self,
        path: &str,
        mut source: R,
    ) -> Result<WriteResult, TransportError>
    where
        R: AsyncRead + Unpin + Send,
    {
        if self.session.try_exists(path).await.map_err(map_sftp_error)? {
            return Err(TransportError::Conflict(path.to_string()));
        }

        let mut file = match self
            .session
            .open_with_flags(
                path,
                OpenFlags::CREATE | OpenFlags::EXCLUDE | OpenFlags::WRITE,
            )
            .await
        {
            Ok(file) => file,
            Err(error) => {
                // EXCLUDE is the race-safe guard. Re-checking existence only classifies
                // a failed exclusive create as a semantic conflict for callers.
                if self.session.try_exists(path).await.unwrap_or(false) {
                    return Err(TransportError::Conflict(path.to_string()));
                }
                return Err(map_sftp_error(error));
            }
        };

        let copied = tokio::io::copy(&mut source, &mut file)
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        file.flush()
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;
        file.sync_all().await.map_err(map_sftp_error)?;
        file.shutdown()
            .await
            .map_err(|error| TransportError::Sftp(error.to_string()))?;

        Ok(WriteResult {
            path: path.to_string(),
            bytes_written: copied,
        })
    }
}

fn map_sftp_error(error: russh_sftp::client::error::Error) -> TransportError {
    TransportError::Sftp(error.to_string())
}
