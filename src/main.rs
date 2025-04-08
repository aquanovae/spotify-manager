mod playlist;
mod spotify;

use anyhow::Result;
use clap::{ Parser, Subcommand };

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
    },
    /// Generate daily playlist
    Generate,
    /// Switch current playing track to selected playlist
    SwitchTrack,
    /// Get current playing track info
    TrackInfo,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("No refresh token found")]
    NoRefreshToken,
    #[error("Error parsing spotify authentication response")]
    ParseAuthResponse,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Auth{ server_login } => {
            spotify::authenticate(server_login).await?;
        },
        _ => run_with_api(cli.command).await?,
    };
    Ok(())
}

async fn run_with_api(command: Command) -> Result<()> {
    let spotify = spotify::get_api().await?;
    match command {
        Command::Generate => {
        },
        Command::SwitchTrack => {
        },
        Command::TrackInfo => {
        },
        _ => (),
    };
    Ok(())
}
