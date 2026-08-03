use clap::Parser;
use std::process::ExitCode;

mod api;
mod auth;
mod client;
mod config;
mod error;
mod format;
mod mcp;

use config::AkeylessMcpConfig;

#[derive(Parser)]
#[command(name = "akeyless_mcp", about = "The purpose of this application is to provide access to Akeyless API.")]
struct Cli {
    /// Run in MCP server mode (default when no subcommand given)
    #[command(subcommand)]
    command: Option<Command>,

    /// DEPRECATED and NOT READ. A key passed here is visible in the
    /// process table to every local user and lands in shell history,
    /// so this flag is not wired to the client — `mcp::AkeylessMcpMcp`
    /// calls `auth::resolve_api_key(None, ..)` on purpose. Set
    /// `AKEYLESS_MCP_API_KEY`, or put the key in the `api_key_file`
    /// named by the config (default `~/.config/akeyless_mcp/api-key`).
    ///
    /// Kept accepted rather than removed so no existing invocation
    /// starts failing to parse; every use warns.
    #[arg(long)]
    api_key: Option<String>,

    /// API base URL (overrides config)
    #[arg(long)]
    api_url: Option<String>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run the MCP server on stdio
    Serve,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    // No subcommand or explicit serve -> MCP server mode (stdio)
    match cli.command {
        None | Some(Command::Serve) => {
            init_tracing(true);
            warn_on_argv_api_key(cli.api_key.as_deref());
            if let Err(e) = mcp::run().await {
                eprintln!("MCP server error: {e}");
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
    }
}

/// Say out loud that `--api-key` was given and was not used.
///
/// Silence here would be worse than the flag itself: an operator who
/// passes a key on the command line has already paid the exposure (the
/// process table, their shell history) and would otherwise get an
/// opaque "no API key" failure from `resolve_api_key`, with no hint
/// that the value they supplied was dropped on the floor.
fn warn_on_argv_api_key(api_key: Option<&str>) {
    if api_key.is_some() {
        tracing::warn!(
            "--api-key is deprecated and is NOT read: a credential on the command \
             line is visible in the process table to every local user and is \
             written to shell history. Set AKEYLESS_MCP_API_KEY, or write the key \
             to the configured api_key_file (default \
             ~/.config/akeyless_mcp/api-key, mode 0600)."
        );
    }
}

fn init_tracing(json: bool) {
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    if json {
        fmt().json().with_env_filter(filter).with_writer(std::io::stderr).init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
