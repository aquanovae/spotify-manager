use crate::spotify::Spotify;

use anyhow::Result;
use enum_iterator::Sequence;
use spotify_rs::model::PlayableItem;
use std::{
    collections::HashMap,
    sync::{ Arc, Mutex },
};

#[derive(Debug, PartialEq, Sequence)]
pub enum Playlist {
    CurrentLoop,
    FreshVibrations,
    IntoTheAbyss,
    FlowingAtmosphere,
    NerveRacking,
    DeepSpaceWubs,
    DailyPlaylist,
}

impl Playlist {
    pub fn id(&self) -> String {
        let id = match self {
            Playlist::CurrentLoop => "77JTZoDLsmXm1ODTdVc1oz",
            Playlist::FreshVibrations => "7tmG3W0fLJw9eDEaRCG8VY",
            Playlist::IntoTheAbyss => "0oc9wsvrxgwI17PCfbEo1l",
            Playlist::FlowingAtmosphere => "4Ty1f3XV2rOPrNOOBMPldQ",
            Playlist::NerveRacking => "1THuBLaWoC0E8PNo2MsFka",
            Playlist::DeepSpaceWubs => "5bQBn71eqqtVqTPsB0XlFf",
            Playlist::DailyPlaylist => "42O1aSlfF0vlmLuBkPlcDO",
        };
        String::from(id)
    }
}

pub struct TrackLists {
    track_lists: HashMap<String, String>,
}

impl TrackLists {
    pub fn new() -> TrackLists {
        TrackLists {
            track_lists: HashMap::new(),
        }
    }

    pub async fn fetch_all(&mut self, spotify: &mut Spotify) -> Result<()> {
        for playlist in enum_iterator::all::<Playlist>() {
            self.fetch(spotify, playlist).await?;
        }
        Ok(())
    }

    pub async fn fetch(
        &mut self, spotify: &mut Spotify, playlist: Playlist
    ) -> Result<()> {
        let track_list = spotify
            .playlist_items(playlist.id())
            .get()
            .await?
            .items
            .iter()
            .filter_map(|item| {
                match &item.track {
                    PlayableItem::Track(track) => Some(track.uri.clone()),
                    _ => None,
                }
            })
            .collect();
        self.track_lists.insert(playlist.id(), track_list);
        Ok(())
    }
}

/*
async fn fetch_track_list(
    spotify: Arc<Mutex<&mut Spotify>>, playlist: Playlist
) -> Result<(Playlist, Vec<String>)> {
    let mut spotify = spotify.lock()?;
    let request = spotify.playlist_items(playlist.id());
    drop(spotify);
    todo!()
}
*/
