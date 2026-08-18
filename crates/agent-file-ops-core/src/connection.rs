use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct ConnectionCapabilities {
    pub sftp: bool,
    pub shell: bool,
    pub commands: BTreeMap<String, bool>,
}

impl ConnectionCapabilities {
    pub fn command_available(&self, command: &str) -> bool {
        self.commands.get(command).copied().unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectionDescriptor {
    pub id: String,
    pub home: String,
    pub sftp_home: String,
    pub shell_home: Option<String>,
    pub aliases: BTreeMap<String, String>,
    pub capabilities: ConnectionCapabilities,
}

impl ConnectionDescriptor {
    pub fn new(
        id: impl Into<String>,
        home: impl Into<String>,
        sftp_home: impl Into<String>,
        shell_home: Option<String>,
        aliases: BTreeMap<String, String>,
        capabilities: ConnectionCapabilities,
    ) -> Self {
        Self {
            id: id.into(),
            home: home.into(),
            sftp_home: sftp_home.into(),
            shell_home,
            aliases,
            capabilities,
        }
    }
}
