mod cache;
mod cli;
mod error;
mod generate;
mod playlist;
mod spotify;
mod switch_track;
mod track_info;

use crate::cli::{ Command, Cli };
use crate::error::Error;
use anyhow::Result;
use clap::Parser;


#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();

    if let Command::TrackInfo{ daemon: false } = cli.command {
        track_info::print_info()?;
        return Ok(());
    }

    if let Command::ListPlaylists = cli.command {
        playlist::print_switchable();
        return Ok(());
    }

    if let Command::Auth{ cli_login } = cli.command {
        spotify::authenticate(cli_login).await?;
        return Ok(());
    }

    let spotify = &mut spotify::get_api().await?;

    match cli.command {
        Command::Generate => {
            generate::daily_playlist(spotify).await?;
        },
        Command::SwitchTrack{ destination, remove } => {
            switch_track::to_playlist(spotify, destination, remove).await?;
        },
        Command::TrackInfo{ .. } => {
            track_info::run_daemon(spotify).await?;
        },
        _ => (),
    }

    Ok(())
}
