use crate::spotify::Spotify;

use anyhow::Result;
use std::collections::HashMap;

pub type TrackLists = HashMap<String, Vec<String>>;

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
    pub fn id(self) -> String {
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

pub async fn fetch_track_list(
    spotify: &mut Spotify, track_lists: &mut TrackLists, playlist: Playlist
) -> Result<TrackLists> {
    spotify
        .playlist_items(playlist.id())
        .get()
        .await?
        .track
    todo!()
}
