mod spotify;

use anyhow::Result;
use clap::{ Parser, Subcommand };
use tokio::runtime::Runtime;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Authenticate user
    Auth {
        #[arg(short, default_value_t = false)]
        server_login: bool,
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("No refresh token found")]
    NoRefreshToken,
    #[error("Error parsing spotify authentication response")]
    ParseAuthResponse,
}

fn main() -> Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let cli = Cli::parse();
        match cli.command {
            Command::Auth{ server_login } => spotify::authenticate(server_login).await?,
        };
        Ok(())
    })
}
