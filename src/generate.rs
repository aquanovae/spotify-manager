use crate::{
    playlist::{ Playlist, PlaylistData },
    spotify::Spotify,
    Error,
};
use anyhow::Result;
use rand::{
    seq::SliceRandom,
    Rng,
};

const DISCOVERY_LENGTH: usize = 30;
const PLAYLIST_LENGTH: usize = 175;
const CHUNK_SIZE: usize = 100;

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

pub async fn daily_playlist(
    spotify: &mut Spotify, track_lists: &mut PlaylistData
) -> Result<()> {
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
        .iter()
        .map(|(_, track_list)| track_list)
        .flatten()
        .collect::<Vec<_>>()
        .triple_shuffle(rng)
        .into_iter()
        .take(PLAYLIST_LENGTH - selection.len())
        .for_each(|track| selection.push(track.clone()));
    println!("{}", daily_playlist.len());
    for chunk in daily_playlist.chunks(CHUNK_SIZE) {
        spotify
            .remove_playlist_items(Playlist::DailyPlaylist.id(), chunk)
            .send()
            .await?;
    }
    /*
    for chunk in selection.triple_shuffle(rng).chunks(CHUNK_SIZE) {
        spotify
            .add_items_to_playlist(Playlist::DailyPlaylist.id(), chunk)
            .send()
            .await?;
    }
    */
    Ok(())
}
