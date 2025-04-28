use crate::playlist::Playlist;

use clap::{ Parser, Subcommand };


#[derive(Debug, Parser)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Command,
}


#[derive(Debug, Subcommand)]
pub enum Command {

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
        #[arg(value_enum)]
        destination: Option<Playlist>,

        /// Remove track
        #[arg(short, default_value_t = false)]
        remove: bool,
    },

    /// Get current playing track info
    TrackInfo {

        /// Run daemon
        #[arg(short, default_value_t = false)]
        daemon: bool,
    },
}

