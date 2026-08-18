use crate::{normalize_path, AgentFileOpsError, ConnectionDescriptor};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolvedPath {
    pub connection: String,
    pub logical_path: String,
    pub sftp_path: String,
    pub shell_path: Option<String>,
    pub base: String,
    pub follow_symlinks: bool,
}

fn join_root(root: &str, relative: &str) -> String {
    let root = root.trim_end_matches('/');
    let relative = relative.trim_start_matches('/');

    if root.is_empty() || root == "." {
        return relative.to_string();
    }
    if relative.is_empty() {
        return root.to_string();
    }
    format!("{root}/{relative}")
}

fn alias_root<'a>(connection: &'a ConnectionDescriptor, base: &str) -> Result<&'a str, AgentFileOpsError> {
    if base == "home" {
        return Ok("");
    }
    connection
        .aliases
        .get(base)
        .map(String::as_str)
        .ok_or_else(|| AgentFileOpsError::UnknownAlias(base.to_string()))
}

pub fn resolve_connection_path(
    connection: &ConnectionDescriptor,
    base: &str,
    path: &str,
    follow_symlinks: bool,
) -> Result<ResolvedPath, AgentFileOpsError> {
    let normalized = normalize_path(base, path, follow_symlinks)?;

    if base == "absolute" {
        if !connection.sftp_home.starts_with('/') {
            return Err(AgentFileOpsError::AbsolutePathUnavailable);
        }
        let shell_path = connection
            .shell_home
            .as_ref()
            .filter(|home| home.starts_with('/'))
            .map(|_| normalized.path.clone());
        return Ok(ResolvedPath {
            connection: connection.id.clone(),
            logical_path: format!("absolute:{}", normalized.path),
            sftp_path: normalized.path,
            shell_path,
            base: base.to_string(),
            follow_symlinks,
        });
    }

    let alias = alias_root(connection, base)?;
    let normalized_alias = normalize_path("home", alias, false)?;
    let logical_relative = if normalized_alias.path.is_empty() {
        normalized.path.clone()
    } else {
        join_root(&normalized_alias.path, &normalized.path)
    };

    let sftp_path = join_root(&connection.sftp_home, &logical_relative);
    let shell_path = connection
        .shell_home
        .as_ref()
        .map(|home| join_root(home, &logical_relative));

    let logical_path = if base == "home" {
        format!("home:{}", normalized.path)
    } else {
        format!("{base}:{}", normalized.path)
    };

    Ok(ResolvedPath {
        connection: connection.id.clone(),
        logical_path,
        sftp_path,
        shell_path,
        base: base.to_string(),
        follow_symlinks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConnectionCapabilities;
    use std::collections::BTreeMap;

    fn connection() -> ConnectionDescriptor {
        let mut aliases = BTreeMap::new();
        aliases.insert("web".into(), "domains/example.com/public_html".into());
        ConnectionDescriptor::new(
            "prod",
            "/home/u1",
            ".",
            Some("/home/u1".into()),
            aliases,
            ConnectionCapabilities {
                sftp: true,
                shell: true,
                commands: BTreeMap::new(),
            },
        )
    }

    #[test]
    fn resolves_alias_in_both_namespaces() {
        let value = resolve_connection_path(&connection(), "web", "releases/app.zip", false).unwrap();
        assert_eq!(value.sftp_path, "domains/example.com/public_html/releases/app.zip");
        assert_eq!(
            value.shell_path.as_deref(),
            Some("/home/u1/domains/example.com/public_html/releases/app.zip")
        );
    }

    #[test]
    fn unknown_alias_fails_closed() {
        assert_eq!(
            resolve_connection_path(&connection(), "missing", "file.txt", false),
            Err(AgentFileOpsError::UnknownAlias("missing".into()))
        );
    }

    #[test]
    fn alias_path_cannot_escape_selected_base() {
        assert_eq!(
            resolve_connection_path(&connection(), "web", "../../secret", false),
            Err(AgentFileOpsError::PathEscape)
        );
    }

    #[test]
    fn absolute_mode_requires_absolute_sftp_namespace() {
        assert_eq!(
            resolve_connection_path(&connection(), "absolute", "/var/www/app", false),
            Err(AgentFileOpsError::AbsolutePathUnavailable)
        );
    }
}
