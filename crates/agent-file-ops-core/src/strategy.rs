use crate::{AgentFileOpsError, ConnectionCapabilities};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendStrategy {
    pub strategy: String,
    pub accelerated: bool,
}

fn accelerated_strategy(
    operation: &str,
    capabilities: &ConnectionCapabilities,
    shell_path_safe: bool,
) -> Option<&'static str> {
    if !capabilities.shell || !shell_path_safe {
        return None;
    }

    match operation {
        "copy" if capabilities.command_available("cp") => Some("shell-cp"),
        "checksum" if capabilities.command_available("sha256sum") => Some("shell-sha256sum"),
        "move" if capabilities.command_available("mv") => Some("shell-mv"),
        _ => None,
    }
}

fn sftp_strategy(operation: &str, capabilities: &ConnectionCapabilities) -> Option<&'static str> {
    if !capabilities.sftp {
        return None;
    }

    match operation {
        "copy" => Some("sftp-stream"),
        "checksum" => Some("sftp-hash"),
        "move" => Some("sftp-rename-or-stream"),
        _ => None,
    }
}

pub fn select_backend_strategy(
    operation: &str,
    capabilities: &ConnectionCapabilities,
    shell_path_safe: bool,
) -> Result<BackendStrategy, AgentFileOpsError> {
    if let Some(strategy) = accelerated_strategy(operation, capabilities, shell_path_safe) {
        return Ok(BackendStrategy {
            strategy: strategy.to_string(),
            accelerated: true,
        });
    }

    if let Some(strategy) = sftp_strategy(operation, capabilities) {
        return Ok(BackendStrategy {
            strategy: strategy.to_string(),
            accelerated: false,
        });
    }

    match operation {
        "copy" | "checksum" | "move" => Err(AgentFileOpsError::CapabilityUnavailable(
            operation.to_string(),
        )),
        other => Err(AgentFileOpsError::UnknownOperation(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn capabilities(sftp: bool, shell: bool, commands: &[&str]) -> ConnectionCapabilities {
        let commands = commands
            .iter()
            .map(|name| ((*name).to_string(), true))
            .collect::<BTreeMap<_, _>>();
        ConnectionCapabilities {
            sftp,
            shell,
            commands,
        }
    }

    #[test]
    fn copy_prefers_safe_shell_acceleration() {
        let result =
            select_backend_strategy("copy", &capabilities(true, true, &["cp"]), true).unwrap();
        assert_eq!(
            result,
            BackendStrategy {
                strategy: "shell-cp".into(),
                accelerated: true,
            }
        );
    }

    #[test]
    fn copy_falls_back_to_sftp_when_shell_mapping_is_unsafe() {
        let result =
            select_backend_strategy("copy", &capabilities(true, true, &["cp"]), false).unwrap();
        assert_eq!(
            result,
            BackendStrategy {
                strategy: "sftp-stream".into(),
                accelerated: false,
            }
        );
    }

    #[test]
    fn checksum_uses_sftp_without_sha256sum() {
        let result =
            select_backend_strategy("checksum", &capabilities(true, true, &[]), true).unwrap();
        assert_eq!(result.strategy, "sftp-hash");
        assert!(!result.accelerated);
    }

    #[test]
    fn no_backend_fails_closed() {
        assert_eq!(
            select_backend_strategy("copy", &capabilities(false, false, &[]), false),
            Err(AgentFileOpsError::CapabilityUnavailable("copy".into()))
        );
    }
}
