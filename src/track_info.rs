use crate::{
    ipc::Socket,
    playlist::{ self, FetchMode, Playlist, TrackLists },
    spotify::Spotify,
    Error,
};
use anyhow::Result;
use spotify_rs::model::PlayableItem;
use std::{
    sync::mpsc,
    time::Duration,
    thread,
};

const POLL_INTERVAL: u64 = 1;

enum Event {
    RefreshTrackLists,
    UpdateOutput,
}

pub async fn run_daemon(
    spotify: &mut Spotify, mut track_lists: TrackLists
) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let socket = Socket::new()?;
    thread::spawn(move || {
        loop {
            tx.send(Event::UpdateOutput).unwrap();
            if socket.poll_signal() {
                tx.send(Event::RefreshTrackLists).unwrap();
            }
            thread::sleep(Duration::from_secs(POLL_INTERVAL));
        }
    });
    while let Ok(event) = rx.recv() {
        match event {
            Event::UpdateOutput => {
                let _ = update_output(spotify, &track_lists).await;
            },
            Event::RefreshTrackLists => {
                track_lists = playlist::get_track_lists(spotify, FetchMode::Limited).await?;
            },
        };
    }
    Ok(())
}

async fn update_output(spotify: &mut Spotify, track_lists: &TrackLists) -> Result<()> {
    let currently_playling = spotify.get_currently_playing_track(None).await?;
    if let Some(PlayableItem::Track(track)) = currently_playling.item {
        let artists = track
            .artists
            .iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>()
            .as_slice()
            .join(", ");
        let playlist_status = playlist_status(track_lists, &track.uri)?;
        println!("{} - {} [{}]", artists, track.name, playlist_status);
    };
    Ok(())
}

fn playlist_status(track_lists: &TrackLists, track_uri: &String) -> Result<String> {
    let mut status = String::new();
    let current_loop = track_lists
        .get(&Playlist::CurrentLoop)
        .ok_or(Error::PlaylistNotFetched)?;
    let fresh_vibrations = track_lists
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
