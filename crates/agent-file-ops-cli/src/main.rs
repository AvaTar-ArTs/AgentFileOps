use agent_file_ops_core::{
    classify_risk, normalize_path, resolve_connection_path, select_backend_strategy,
    AgentFileOpsError, ConnectionCapabilities, ConnectionDescriptor,
};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Parser)]
#[command(name = "afo")]
#[command(about = "AgentFileOps native conformance harness")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    NormalizePath {
        #[arg(long)]
        base: String,
        #[arg(long)]
        path: String,
        #[arg(long, default_value_t = false)]
        follow_symlinks: bool,
    },
    ClassifyRisk {
        #[arg(long)]
        operation: String,
    },
    ResolveConnectionPath {
        #[arg(long)]
        connection: String,
        #[arg(long)]
        home: String,
        #[arg(long)]
        sftp_home: String,
        #[arg(long)]
        shell_home: Option<String>,
        #[arg(long = "alias")]
        aliases: Vec<String>,
        #[arg(long)]
        base: String,
        #[arg(long)]
        path: String,
        #[arg(long, default_value_t = false)]
        follow_symlinks: bool,
    },
    SelectBackendStrategy {
        #[arg(long)]
        operation: String,
        #[arg(long, default_value_t = false)]
        sftp: bool,
        #[arg(long, default_value_t = false)]
        shell: bool,
        #[arg(long, default_value_t = false)]
        shell_path_safe: bool,
        #[arg(long = "command")]
        commands: Vec<String>,
    },
}

#[derive(Serialize)]
struct RiskResponse<'a> {
    operation: &'a str,
    risk: agent_file_ops_core::RiskLevel,
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    code: &'a str,
    message: String,
}

fn emit_error(error: AgentFileOpsError) -> ! {
    let body = ErrorResponse {
        code: error.code(),
        message: error.to_string(),
    };
    eprintln!("{}", serde_json::to_string(&body).expect("error serialization"));
    std::process::exit(2);
}

fn emit_cli_error(code: &'static str, message: impl Into<String>) -> ! {
    let body = ErrorResponse {
        code,
        message: message.into(),
    };
    eprintln!("{}", serde_json::to_string(&body).expect("error serialization"));
    std::process::exit(2);
}

fn parse_aliases(values: Vec<String>) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    for value in values {
        let Some((name, path)) = value.split_once('=') else {
            emit_cli_error("invalid_alias", format!("alias must use name=path syntax: {value}"));
        };
        if name.is_empty() {
            emit_cli_error("invalid_alias", "alias name must not be empty");
        }
        aliases.insert(name.to_string(), path.to_string());
    }
    aliases
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::NormalizePath {
            base,
            path,
            follow_symlinks,
        } => match normalize_path(&base, &path, follow_symlinks) {
            Ok(value) => println!("{}", serde_json::to_string(&value).expect("path serialization")),
            Err(error) => emit_error(error),
        },
        Command::ClassifyRisk { operation } => match classify_risk(&operation) {
            Ok(risk) => {
                let body = RiskResponse {
                    operation: &operation,
                    risk,
                };
                println!("{}", serde_json::to_string(&body).expect("risk serialization"));
            }
            Err(error) => emit_error(error),
        },
        Command::ResolveConnectionPath {
            connection,
            home,
            sftp_home,
            shell_home,
            aliases,
            base,
            path,
            follow_symlinks,
        } => {
            let descriptor = ConnectionDescriptor::new(
                connection,
                home,
                sftp_home,
                shell_home,
                parse_aliases(aliases),
                ConnectionCapabilities::default(),
            );
            match resolve_connection_path(&descriptor, &base, &path, follow_symlinks) {
                Ok(value) => println!(
                    "{}",
                    serde_json::to_string(&value).expect("resolved path serialization")
                ),
                Err(error) => emit_error(error),
            }
        }
        Command::SelectBackendStrategy {
            operation,
            sftp,
            shell,
            shell_path_safe,
            commands,
        } => {
            let commands = commands
                .into_iter()
                .map(|name| (name, true))
                .collect::<BTreeMap<_, _>>();
            let capabilities = ConnectionCapabilities {
                sftp,
                shell,
                commands,
            };
            match select_backend_strategy(&operation, &capabilities, shell_path_safe) {
                Ok(value) => println!(
                    "{}",
                    serde_json::to_string(&value).expect("strategy serialization")
                ),
                Err(error) => emit_error(error),
            }
        }
    }
}
