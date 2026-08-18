use agent_file_ops_core::{
    classify_risk, normalize_path, resolve_connection_path, select_backend_strategy,
    AgentFileOpsError, ConnectionCapabilities, ConnectionDescriptor,
};
use agent_file_ops_ssh::{
    AgentFileOpsSftp, AgentFileOpsSshSession, RemoteStat, SshTransportConfig, TransportError,
};
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

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
    Sftp {
        #[command(subcommand)]
        command: SftpCommand,
    },
}

#[derive(Subcommand)]
enum SftpCommand {
    List(SftpPathArgs),
    Stat(SftpPathArgs),
    Read(SftpReadArgs),
    WriteNew(SftpWriteArgs),
}

#[derive(Args, Clone)]
struct SftpConnectionArgs {
    #[arg(long)]
    host: String,
    #[arg(long, default_value_t = 22)]
    port: u16,
    #[arg(long)]
    username: String,
    #[arg(long)]
    known_hosts: String,
    #[arg(long)]
    credential: String,
    #[arg(long, default_value_t = agent_file_ops_ssh::MAX_INLINE_READ_BYTES)]
    inline_read_bytes: u64,
}

#[derive(Args, Clone)]
struct SftpPathArgs {
    #[command(flatten)]
    connection: SftpConnectionArgs,
    #[arg(long)]
    path: String,
    #[arg(long)]
    limit: Option<usize>,
}

#[derive(Args, Clone)]
struct SftpReadArgs {
    #[command(flatten)]
    connection: SftpConnectionArgs,
    #[arg(long)]
    path: String,
    #[arg(long, default_value_t = 0)]
    offset: u64,
    #[arg(long)]
    limit: u64,
}

#[derive(Args, Clone)]
struct SftpWriteArgs {
    #[command(flatten)]
    connection: SftpConnectionArgs,
    #[arg(long)]
    path: String,
    #[arg(long, value_name = "FILE")]
    data_file: PathBuf,
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

#[derive(Serialize)]
struct ReadResponse {
    path: String,
    offset: u64,
    bytes: Vec<u8>,
}

fn emit_error(error: AgentFileOpsError) -> ! {
    let body = ErrorResponse {
        code: error.code(),
        message: error.to_string(),
    };
    eprintln!(
        "{}",
        serde_json::to_string(&body).expect("error serialization")
    );
    std::process::exit(2);
}

fn emit_cli_error(code: &'static str, message: impl Into<String>) -> ! {
    let body = ErrorResponse {
        code,
        message: message.into(),
    };
    eprintln!(
        "{}",
        serde_json::to_string(&body).expect("error serialization")
    );
    std::process::exit(2);
}

fn emit_transport_error(error: TransportError) -> ! {
    let body = ErrorResponse {
        code: "transport_error",
        message: error.to_string(),
    };
    eprintln!(
        "{}",
        serde_json::to_string(&body).expect("error serialization")
    );
    std::process::exit(2);
}

async fn open_sftp(
    args: &SftpConnectionArgs,
) -> Result<(AgentFileOpsSshSession, AgentFileOpsSftp), TransportError> {
    let config =
        SshTransportConfig::new(&args.host, args.port, &args.known_hosts, &args.credential)
            .with_username(&args.username)
            .with_inline_read_limit(args.inline_read_bytes);
    let mut session = AgentFileOpsSshSession::new(config);
    session.connect().await?;
    let sftp = session.open_sftp().await?;
    Ok((session, sftp))
}

fn print_stat(stat: RemoteStat) {
    println!(
        "{}",
        serde_json::to_string(&stat).expect("stat serialization")
    );
}

fn parse_aliases(values: Vec<String>) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    for value in values {
        let Some((name, path)) = value.split_once('=') else {
            emit_cli_error(
                "invalid_alias",
                format!("alias must use name=path syntax: {value}"),
            );
        };
        if name.is_empty() {
            emit_cli_error("invalid_alias", "alias name must not be empty");
        }
        aliases.insert(name.to_string(), path.to_string());
    }
    aliases
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::NormalizePath {
            base,
            path,
            follow_symlinks,
        } => match normalize_path(&base, &path, follow_symlinks) {
            Ok(value) => println!(
                "{}",
                serde_json::to_string(&value).expect("path serialization")
            ),
            Err(error) => emit_error(error),
        },
        Command::ClassifyRisk { operation } => match classify_risk(&operation) {
            Ok(risk) => {
                let body = RiskResponse {
                    operation: &operation,
                    risk,
                };
                println!(
                    "{}",
                    serde_json::to_string(&body).expect("risk serialization")
                );
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
        Command::Sftp { command } => match command {
            SftpCommand::List(args) => {
                let (mut session, sftp) = open_sftp(&args.connection)
                    .await
                    .unwrap_or_else(|error| emit_transport_error(error));
                let result = sftp.list(&args.path, args.limit).await;
                let _ = session.disconnect().await;
                match result {
                    Ok(value) => println!(
                        "{}",
                        serde_json::to_string(&value).expect("list serialization")
                    ),
                    Err(error) => emit_transport_error(error),
                }
            }
            SftpCommand::Stat(args) => {
                let (mut session, sftp) = open_sftp(&args.connection)
                    .await
                    .unwrap_or_else(|error| emit_transport_error(error));
                let result = sftp.stat(&args.path).await;
                let _ = session.disconnect().await;
                match result {
                    Ok(value) => print_stat(value),
                    Err(error) => emit_transport_error(error),
                }
            }
            SftpCommand::Read(args) => {
                let (mut session, sftp) = open_sftp(&args.connection)
                    .await
                    .unwrap_or_else(|error| emit_transport_error(error));
                let result = sftp.read(&args.path, args.offset, args.limit).await;
                let _ = session.disconnect().await;
                match result {
                    Ok(bytes) => println!(
                        "{}",
                        serde_json::to_string(&ReadResponse {
                            path: args.path,
                            offset: args.offset,
                            bytes,
                        })
                        .expect("read serialization")
                    ),
                    Err(error) => emit_transport_error(error),
                }
            }
            SftpCommand::WriteNew(args) => {
                let data = std::fs::read(&args.data_file).unwrap_or_else(|error| {
                    emit_cli_error(
                        "local_input_error",
                        format!("failed to read {}: {error}", args.data_file.display()),
                    )
                });
                let (mut session, sftp) = open_sftp(&args.connection)
                    .await
                    .unwrap_or_else(|error| emit_transport_error(error));
                let result = sftp.write_new(&args.path, &data).await;
                let _ = session.disconnect().await;
                match result {
                    Ok(value) => println!(
                        "{}",
                        serde_json::to_string(&value).expect("write serialization")
                    ),
                    Err(error) => emit_transport_error(error),
                }
            }
        },
    }
}
