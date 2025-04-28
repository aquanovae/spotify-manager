use crate::Error;
use crate::playlist::{ self, FetchMode, Playlist, TrackLists };
use crate::spotify::Spotify;

use anyhow::Result;
use serde::{ Deserialize, Serialize };
use spotify_rs::model::PlayableItem;
use std::io::Write;
use std::fs;
use std::net::Shutdown;
use std::os::unix::net::{ UnixListener, UnixStream };
use std::sync::mpsc;
use std::time::Duration;
use std::thread;


const POLL_INTERVAL: u64 = 1;
const SOCKET_PATH: &str = "/tmp/spotify-manager/socket";


#[derive(Deserialize, Serialize)]
enum Message {
    RefreshTrackLists,
    RequestInfo,
    TrackInfo{ track_info: String },
}


enum Event {
    RefreshTrackLists,
    SendInfo{ stream: UnixStream },
    UpdateOutput,
}


pub fn print_info() -> Result<()> {

    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let message = ron::ser::to_string(&Message::RequestInfo)?;

    stream.write_all(message.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    if let Message::TrackInfo{ track_info } = ron::de::from_reader(stream)? {
        println!("{}", track_info);
    }

    Ok(())
}


pub fn request_refresh() -> Result<()> {

    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let message = ron::ser::to_string(&Message::RefreshTrackLists)?;
    stream.write_all(message.as_bytes())?;

    Ok(())
}


pub async fn run_daemon(spotify: &mut Spotify) -> Result<()> {

    let _ = fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)?;
    listener.set_nonblocking(true)?;

    let (tx, rx) = mpsc::channel();
    let _ = tx.send(Event::UpdateOutput);
    let _ = tx.send(Event::RefreshTrackLists);

    thread::spawn(move || {
        loop {
            let _ = tx.send(Event::UpdateOutput);

            while let Some(event) = poll_events(&listener) {
                let _ = tx.send(event);
            }

            thread::sleep(Duration::from_secs(POLL_INTERVAL));
        }
    });

    let mut track_lists = playlist::get_track_lists(spotify, FetchMode::Cache).await?;
    let mut track_info = String::new();

    while let Ok(event) = rx.recv() {
        match event {
            Event::RefreshTrackLists => {
                refresh_track_lists(spotify, &mut track_lists).await;
            },
            Event::SendInfo{ stream } => {
                send_info(stream, track_info.clone());
            },
            Event::UpdateOutput => {
                update_track_info(spotify, &track_lists, &mut track_info).await;
            },
        }
    }

    Ok(())
}


fn poll_events(listener: &UnixListener) -> Option<Event> {

    let (stream, _) = listener.accept().ok()?;
    let message: Message = ron::de::from_reader(&stream).ok()?;

    match message {
        Message::RefreshTrackLists => Some(Event::RefreshTrackLists),
        Message::RequestInfo => Some(Event::SendInfo{ stream }),
        _ => None,
    }
}


async fn refresh_track_lists(spotify: &mut Spotify, track_lists: &mut TrackLists) {

    match playlist::get_track_lists(spotify, FetchMode::Limited).await {
        Ok(new_track_lists) => *track_lists = new_track_lists,
        Err(_) => (),
    }
}


fn send_info(mut stream: UnixStream, track_info: String) {

    let message = Message::TrackInfo{ track_info };

    if let Ok(message) = ron::ser::to_string(&message) {
        let _ = stream.write_all(message.as_bytes());
    }
}


async fn update_track_info(
    spotify: &mut Spotify, track_lists: &TrackLists, track_info: &mut String
) {

    match fetch_track_info(spotify, track_lists).await {
        Ok(info) => *track_info = info,
        Err(_) => *track_info = String::from("-"),
    }
}


async fn fetch_track_info(
    spotify: &mut Spotify, track_lists: &TrackLists
) -> Result<String> {

    let track = spotify
        .get_currently_playing_track(None)
        .await?
        .item
        .map(|item| match item {
            PlayableItem::Track(track) => Some(track),
            _ => None,
        })
        .flatten()
        .ok_or(Error::NoTrackPlaying)?;

    let artists = track
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>()
        .as_slice()
        .join(", ");

    let playlist_status = playlist_status(track_lists, &track.uri)?;

    let track_info = format!("{} - {} [{}]", artists, track.name, playlist_status);

    Ok(track_info)
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
