use crate::{
    ipc,
    playlist::Playlist,
    spotify::Spotify,
    Error,
};
use anyhow::Result;
use spotify_rs::model::PlayableItem;

pub async fn to_playlist(spotify: &mut Spotify, destination: Option<Playlist>) -> Result<()> {
    let currently_playling = spotify
        .get_currently_playing_track(None)
        .await?
        .item
        .map(|item| match item {
            PlayableItem::Track(track) => Some(track),
            _ => None,
        })
        .flatten()
        .ok_or(Error::NoTrackPlaying)?;
    let id = &[currently_playling.id];
    let uri = &[currently_playling.uri];
    match destination {
        Some(playlist) => {
            spotify.save_tracks(id).await?;
            spotify.add_items_to_playlist(playlist.id(), uri).send().await?;
            if playlist != Playlist::CurrentLoop {
                spotify.remove_playlist_items(Playlist::CurrentLoop.id(), uri).send().await?;
            }
        },
        None => (),
    };
    spotify.remove_playlist_items(Playlist::FreshVibrations.id(), uri).send().await?;
    ipc::send_signal()?;
    Ok(())
}
