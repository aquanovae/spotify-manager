use crate::Error;
use crate::playlist::{ self, FetchMode, Playlist };
use crate::spotify::{ CHUNK_SIZE, Spotify };

use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;


const DISCOVERY_LENGTH: usize = 30;
const PLAYLIST_LENGTH: usize = 175;


trait TripleShuffle {
    fn triple_shuffle<R>(self, rng: &mut R) -> Self
    where
        R: Rng + ?Sized;
}

impl<T> TripleShuffle for Vec<T> {
    fn triple_shuffle<R>(mut self, rng: &mut R) -> Self
    where
        R: Rng + ?Sized
    {
        self.shuffle(rng);
        self.shuffle(rng);
        self.shuffle(rng);
        self
    }
}


pub async fn daily_playlist(spotify: &mut Spotify) -> Result<()> {

    let mut track_lists = playlist::get_track_lists(spotify, FetchMode::All).await?;
    let rng = &mut rand::rng();

    let daily_playlist = track_lists
        .remove(&Playlist::DailyPlaylist)
        .ok_or(Error::PlaylistNotFetched)?;

    let mut selection = track_lists
        .remove(&Playlist::CurrentLoop)
        .ok_or(Error::PlaylistNotFetched)?;

    track_lists
        .remove(&Playlist::FreshVibrations)
        .ok_or(Error::PlaylistNotFetched)?
        .triple_shuffle(rng)
        .into_iter()
        .take(DISCOVERY_LENGTH)
        .for_each(|track| selection.push(track));

    track_lists
        .into_iter()
        .map(|(_, track_list)| track_list)
        .flatten()
        .collect::<Vec<_>>()
        .triple_shuffle(rng)
        .into_iter()
        .take(PLAYLIST_LENGTH - selection.len())
        .for_each(|track| selection.push(track));

    for chunk in daily_playlist.chunks(CHUNK_SIZE) {
        spotify
            .remove_playlist_items(Playlist::DailyPlaylist.id(), chunk)
            .send()
            .await?;
    }

    for chunk in selection.triple_shuffle(rng).chunks(CHUNK_SIZE) {
        spotify
            .add_items_to_playlist(Playlist::DailyPlaylist.id(), chunk)
            .send()
            .await?;
    }

    Ok(())
}
