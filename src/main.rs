mod cache;
mod error;
mod generate;
mod ipc;
mod playlist;
mod spotify;
mod switch_track;
mod track_info;

pub use crate::error::Error;

use crate::playlist::{ FetchMode, Playlist };
use anyhow::Result;
use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, PartialEq, Subcommand)]
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
    SwitchTrack{
        /// Destination playlist
        #[arg(value_enum)]
        destination: Option<Playlist>,
        /// Remove track
        #[arg(short, default_value_t = false)]
        remove: bool,
    },
    /// Get current playing track info
    TrackInfo,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Command::Auth{ cli_login } = cli.command {
        spotify::authenticate(cli_login).await?;
        return Ok(());
    }
    if cli.command == Command::ListPlaylists {
        let list = enum_iterator::all::<Playlist>()
            .map(|playlist| format!("{}", playlist))
            .filter(|playlist| !playlist.is_empty())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", list);
        return Ok(());
    }
    let spotify = &mut spotify::get_api().await?;
    let track_lists = if cli.command == Command::Generate {
        playlist::get_track_lists(spotify, FetchMode::All).await?
    } else {
        playlist::get_track_lists(spotify, FetchMode::Cache).await?
    };
    match cli.command {
        Command::Generate => {
            generate::daily_playlist(spotify, track_lists).await?;
        },
        Command::SwitchTrack{ destination, remove } => {
            switch_track::to_playlist(spotify, destination, remove).await?;
        },
        Command::TrackInfo => {
            track_info::run_daemon(spotify, track_lists).await?;
        },
        _ => (),
    };
    Ok(())
}
