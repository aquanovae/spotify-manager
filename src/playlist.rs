use crate::{
    cache,
    spotify:: Spotify
};
use anyhow::Result;
use clap::ValueEnum;
use enum_iterator::Sequence;
use serde::{ Deserialize, Serialize };
use spotify_rs::model::PlayableItem;
use std::{
    collections::HashMap,
    fmt::{ self, Display, Formatter },
    ops::{ Deref, DerefMut },
};

type TrackLists = HashMap<Playlist, Vec<String>>;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Sequence, Serialize, ValueEnum)]
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

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Playlist::CurrentLoop => write!(f, "current-loop"),
            Playlist::FreshVibrations => write!(f, "fresh-vibrations"),
            Playlist::IntoTheAbyss => write!(f, "into-the-abyss"),
            Playlist::FlowingAtmosphere => write!(f, "flowing-atmosphere"),
            Playlist::NerveRacking => write!(f, "nerve-racking"),
            Playlist::DeepSpaceWubs => write!(f, "deep-space-wubs"),
            Playlist::DailyPlaylist => write!(f, ""),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlaylistData {
    track_lists: TrackLists,
}

impl PlaylistData {
    pub async fn fetch(spotify: &mut Spotify) -> Result<PlaylistData> {
        let mut track_lists = PlaylistData {
            track_lists: TrackLists::new(),
        };
        for playlist in enum_iterator::all::<Playlist>() {
            track_lists.fetch_track_list(spotify, playlist).await?;
        }
        Ok(track_lists)
    }

    pub fn from_cache() -> Result<PlaylistData> {
        let track_lists = cache::read_track_lists()?;
        Ok(track_lists)
    }

    pub fn write_to_cache(&self) -> Result<()> {
        cache::write_track_lists(self)?;
        Ok(())
    }

    async fn fetch_track_list(
        &mut self, spotify: &mut Spotify, playlist: Playlist
    ) -> Result<()> {
        let mut track_list = Vec::new();
        let get_track_uris = |item| {
            match &item.track {
                PlayableItem::Track(track) => Some(track.uri.clone()),
                _ => None,
            }
        };
        let response = spotify
            .playlist_items(playlist.id())
            .get()
            .await?;
            /*
            .items
            .iter()
            .filter_map(|item| {
                match &item.track {
                    PlayableItem::Track(track) => Some(track.uri.clone()),
                    _ => None,
                }
            })
            .collect();
        */
        println!("{:?}", track_list);
        //self.track_lists.insert(playlist, track_list);
        Ok(())
    }
}

impl Deref for PlaylistData {
    type Target = TrackLists;
    fn deref(&self) -> &Self::Target {
        &self.track_lists
    }
}

impl DerefMut for PlaylistData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.track_lists
    }
}
