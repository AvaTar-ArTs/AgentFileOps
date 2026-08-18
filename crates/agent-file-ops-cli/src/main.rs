use agent_file_ops_core::{classify_risk, normalize_path, AgentFileOpsError};
use clap::{Parser, Subcommand};
use serde::Serialize;

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
    }
}
