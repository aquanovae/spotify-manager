use crate::{
    playlist::{ Playlist, PlaylistData },
    spotify::Spotify,
    Error,
};
use anyhow::Result;
use spotify_rs::model::PlayableItem;
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
        let currently_playling = spotify.get_currently_playing_track(None).await?;
        if let Some(PlayableItem::Track(track)) = currently_playling.item {
            let artists = track
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect::<Vec<String>>()
                .as_slice()
                .join(", ");
            let playlist_status = playlist_status(&playlist_data, &track.uri)?;
            println!("{} - {} <{}>", artists, track.name, playlist_status);
        };
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

fn playlist_status(playlist_data: &PlaylistData, track_uri: &String) -> Result<String> {
    let mut status = String::new();
    let current_loop = playlist_data
        .get(&Playlist::CurrentLoop)
        .ok_or(Error::PlaylistNotFetched)?;
    let fresh_vibrations = playlist_data
        .get(&Playlist::FreshVibrations)
        .ok_or(Error::PlaylistNotFetched)?;
    if current_loop.contains(track_uri) {
        status.push('C');
    }
    if fresh_vibrations.contains(track_uri) {
        status.push('F');
    }
    Ok(status)
}
