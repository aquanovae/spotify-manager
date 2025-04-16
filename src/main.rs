mod cache;
mod generate;
mod playlist;
mod spotify;

use crate::playlist::{ Playlist, PlaylistData };
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
        /// Login in cli mode
        #[arg(short, default_value_t = false)]
        cli_login: bool,
    },
    /// Generate daily playlist
    Generate,
    /// List playlists
    ListPlaylists,
    /// Switch current playing track to selected playlist
    SwitchTrack {
        /// Destination playlist
        destination: Playlist,
    },
    /// Get current playing track info
    TrackInfo,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("No refresh token found")]
    NoRefreshToken,
    #[error("Error parsing spotify authentication response")]
    ParseAuthResponse,
    #[error("Playlist not fetched")]
    PlaylistNotFetched,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Command::Auth{ cli_login } = cli.command {
        spotify::authenticate(cli_login).await?;
        return Ok(());
    };
    let mut spotify = spotify::get_api().await?;
    let mut track_lists = match cli.command {
        Command::SwitchTrack{ .. } => {
            PlaylistData::from_cache()?
        },
        _ => {
            PlaylistData::fetch(&mut spotify).await?
        },
    };
    match cli.command {
        Command::Generate => {
            //generate::daily_playlist(&mut spotify, &mut track_lists).await?;
            track_lists.iter().for_each(|(key, val)| println!("{} {}", key, val.len()));
        },
        Command::ListPlaylists => {
            println!("{}", Playlist::CurrentLoop);
        },
        Command::SwitchTrack{ destination } => {
        },
        Command::TrackInfo => {
        },
        _ => (),
    };
    Ok(())
}
