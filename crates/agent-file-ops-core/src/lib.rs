mod connection;
mod path_resolution;
mod strategy;

pub use connection::{ConnectionCapabilities, ConnectionDescriptor};
pub use path_resolution::{resolve_connection_path, ResolvedPath};
pub use strategy::{select_backend_strategy, BackendStrategy};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NormalizedPath {
    pub base: String,
    pub path: String,
    pub follow_symlinks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RiskLevel {
    #[serde(rename = "level_0")]
    Level0,
    #[serde(rename = "level_1")]
    Level1,
    #[serde(rename = "level_2")]
    Level2,
    #[serde(rename = "level_3")]
    Level3,
    #[serde(rename = "level_4")]
    Level4,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AgentFileOpsError {
    #[error("relative path attempted to escape its selected base")]
    PathEscape,
    #[error("absolute mode requires a path beginning with '/'")]
    AbsolutePathRequired,
    #[error("absolute path mode is unavailable for the selected backend namespace")]
    AbsolutePathUnavailable,
    #[error("relative path bases do not accept absolute paths")]
    RelativePathRequired,
    #[error("path contains a NUL byte")]
    InvalidPath,
    #[error("unknown AgentFileOps alias: {0}")]
    UnknownAlias(String),
    #[error("required AgentFileOps backend capability is unavailable for operation: {0}")]
    CapabilityUnavailable(String),
    #[error("unknown AgentFileOps operation: {0}")]
    UnknownOperation(String),
}

impl AgentFileOpsError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PathEscape => "path_escape",
            Self::AbsolutePathRequired => "absolute_path_required",
            Self::AbsolutePathUnavailable => "absolute_path_unavailable",
            Self::RelativePathRequired => "relative_path_required",
            Self::InvalidPath => "invalid_path",
            Self::UnknownAlias(_) => "unknown_alias",
            Self::CapabilityUnavailable(_) => "capability_unavailable",
            Self::UnknownOperation(_) => "unknown_operation",
        }
    }
}

pub fn normalize_path(
    base: &str,
    path: &str,
    follow_symlinks: bool,
) -> Result<NormalizedPath, AgentFileOpsError> {
    if path.contains('\0') {
        return Err(AgentFileOpsError::InvalidPath);
    }

    let absolute = base == "absolute";
    if absolute && !path.starts_with('/') {
        return Err(AgentFileOpsError::AbsolutePathRequired);
    }
    if !absolute && path.starts_with('/') {
        return Err(AgentFileOpsError::RelativePathRequired);
    }

    let mut segments: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.pop().is_none() {
                    return Err(AgentFileOpsError::PathEscape);
                }
            }
            other => segments.push(other),
        }
    }

    let normalized = if absolute {
        if segments.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", segments.join("/"))
        }
    } else {
        segments.join("/")
    };

    Ok(NormalizedPath {
        base: base.to_string(),
        path: normalized,
        follow_symlinks,
    })
}

pub fn classify_risk(operation: &str) -> Result<RiskLevel, AgentFileOpsError> {
    let level = match operation {
        "list" | "stat" | "find" | "read" | "checksum" => RiskLevel::Level0,
        "mkdir" | "touch" | "write-new" | "copy-new" => RiskLevel::Level1,
        "overwrite" | "move" | "rename" | "chmod" | "symlink" => RiskLevel::Level2,
        "delete" => RiskLevel::Level3,
        "recursive-delete" | "sync-delete" | "bulk-delete" => RiskLevel::Level4,
        other => return Err(AgentFileOpsError::UnknownOperation(other.to_string())),
    };
    Ok(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_relative_segments() {
        let value =
            normalize_path("home", "domains/./avatararts.org/assets/../releases", false).unwrap();
        assert_eq!(value.path, "domains/avatararts.org/releases");
    }

    #[test]
    fn relative_path_cannot_escape_base() {
        assert_eq!(
            normalize_path("home", "../../etc/passwd", false),
            Err(AgentFileOpsError::PathEscape)
        );
    }

    #[test]
    fn absolute_mode_requires_absolute_input() {
        assert_eq!(
            normalize_path("absolute", "home/user/file", false),
            Err(AgentFileOpsError::AbsolutePathRequired)
        );
    }

    #[test]
    fn unknown_operation_fails_closed() {
        assert_eq!(
            classify_risk("shell-anything"),
            Err(AgentFileOpsError::UnknownOperation("shell-anything".into()))
        );
    }
}
