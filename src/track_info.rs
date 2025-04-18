use crate::{
    playlist::PlaylistData,
    spotify::Spotify,
};
use anyhow::Result;
use std::{
    sync::mpsc::{ self, Sender },
    time::Duration,
    thread,
};

const POLL_INTERVAL: u64 = 1;

struct Poll;

pub async fn run_daemon(
    spotify: &mut Spotify, playlist_data: PlaylistData
) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    poll_loop(tx.clone());
    drop(tx);
    while let Ok(_) = rx.recv() {
        let currently_playling = spotify
            .get_currently_playing_track(None).await?;
    }
    Ok(())
}

fn poll_loop(tx: Sender<Poll>) {
    thread::spawn(move || {
        loop {
            tx.send(Poll).unwrap();
            thread::sleep(Duration::from_secs(POLL_INTERVAL));
        }
    });
}
