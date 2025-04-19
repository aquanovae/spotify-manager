mod cache;
mod generate;
mod ipc;
mod playlist;
mod spotify;
mod switch_track;
mod track_info;

use crate::playlist::{ FetchMode, Playlist, TrackLists };
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
    match cli.command {
        Command::Auth{ cli_login } => {
            spotify::authenticate(cli_login).await?;
            return Ok(());
        },
        Command::ListPlaylists => {
            for playlist in enum_iterator::all::<Playlist>() {
                println!("{}", playlist);
            }
            return Ok(());
        },
        _ => (),
    };
    let spotify = &mut spotify::get_api().await?;
    let track_lists = match cli.command {
        Command::Generate => {
            playlist::get_track_lists(spotify, FetchMode::All).await?
        },
        _ => {
            playlist::get_track_lists(spotify, FetchMode::Limited).await?
        },
    };
    match cli.command {
        Command::Generate => {
            generate::daily_playlist(spotify, track_lists).await?;
        },
        Command::SwitchTrack{ destination } => {
            switch_track::to_destination()?;
        },
        Command::TrackInfo => {
            track_info::run_daemon(spotify, track_lists).await?;
        },
        _ => (),
    };
    Ok(())
}
