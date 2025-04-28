use crate::cache;
use crate::spotify::{ CHUNK_SIZE, Spotify };

use anyhow::Result;
use clap::ValueEnum;
use enum_iterator::Sequence;
use serde::{ Deserialize, Serialize };
use spotify_rs::model::PlayableItem;
use std::collections::HashMap;


pub type TrackLists = HashMap<Playlist, Vec<String>>;


#[derive(
    Clone, Debug, Deserialize, Eq, Hash, PartialEq, Sequence, Serialize, ValueEnum
)]
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


pub fn print_switchable() {

    println!("current-loop");
    println!("into-the-abyss");
    println!("flowing-atmosphere");
    println!("nerve-racking");
    println!("deep-space-wubs");
}


pub enum FetchMode {
    All,
    Limited,
    Cache,
}


pub async fn get_track_lists(
    spotify: &mut Spotify, mode: FetchMode
) -> Result<TrackLists> {

    let track_lists = match mode {
        FetchMode::All => {
            fetch_all(spotify).await?
        },
        FetchMode::Limited => {
            fetch_limited(spotify).await?
        },
        FetchMode::Cache => {
            from_cache_or_fetch(spotify).await?
        },
    };

    Ok(track_lists)
}


async fn fetch_all(spotify: &mut Spotify) -> Result<TrackLists> {

    let mut track_lists = TrackLists::new();

    for playlist in enum_iterator::all::<Playlist>() {
        fetch_track_list(spotify, &mut track_lists, playlist).await?;
    }

    Ok(track_lists)
}


async fn from_cache_or_fetch(spotify: &mut Spotify) -> Result<TrackLists> {

    let track_lists = match cache::read_track_lists() {
        Ok(track_lists) => track_lists,
        Err(_) => fetch_limited(spotify).await?,
    };

    Ok(track_lists)
}


async fn fetch_limited(spotify: &mut Spotify) -> Result<TrackLists> {

    let mut track_lists = TrackLists::new();

    fetch_track_list(spotify, &mut track_lists, Playlist::CurrentLoop).await?;
    fetch_track_list(spotify, &mut track_lists, Playlist::FreshVibrations).await?;

    cache::write_track_lists(&track_lists)?;

    Ok(track_lists)
}


async fn fetch_track_list(
    spotify: &mut Spotify, track_lists: &mut TrackLists, playlist: Playlist
) -> Result<()> {

    let mut track_list = Vec::new();
    let mut offset = 0;

    loop {
        let request = spotify
            .playlist_items(playlist.id())
            .offset(offset)
            .get()
            .await?;

        request
            .items
            .iter()
            .filter_map(|item| match &item.track {
                PlayableItem::Track(track) => Some(track.uri.clone()),
                _ => None,
            })
            .for_each(|track| track_list.push(track));

        if request.next.is_none() {
            break;
        }

        offset += CHUNK_SIZE as u32;
    }

    track_lists.insert(playlist, track_list);

    Ok(())
}
