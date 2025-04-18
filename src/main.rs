mod cache;
mod generate;
mod playlist;
mod spotify;
mod track_info;

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
    if let Command::ListPlaylists = cli.command {
        for playlist in enum_iterator::all::<Playlist>() {
            println!("{}", playlist);
        }
        return Ok(());
    };
    let mut spotify = spotify::get_api().await?;
    if let Command::SwitchTrack{ destination } = cli.command {
        return Ok(());
    };
    let playlist_data = PlaylistData::fetch(&mut spotify).await?;
    if let Command::Generate = cli.command {
        generate::daily_playlist(&mut spotify, playlist_data).await?;
        return Ok(());
    };
    if let Command::TrackInfo = cli.command {
        track_info::run_daemon(&mut spotify, playlist_data).await?;
        return Ok(());
    };
    Ok(())
}
