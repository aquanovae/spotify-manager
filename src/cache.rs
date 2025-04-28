use crate::playlist::TrackLists;

use anyhow::Result;
use serde::Serialize;
use std::io::Write;
use std::fs::{ self, File };
use std::path::Path;


const CACHE_PATH: &str = "/tmp/spotify-manager";
const TOKEN_CACHE: &str = "/tmp/spotify-manager/token.ron";
const TRACK_LISTS_CACHE: &str = "/tmp/spotify-manager/track_lists.ron";


pub fn write_token(token: &str) -> Result<()> {

    write_to_cache(TOKEN_CACHE, &token)?;

    Ok(())
}


pub fn read_token() -> Result<String> {

    let token = ron::de::from_reader(File::open(TOKEN_CACHE)?)?;

    Ok(token)
}


pub fn write_track_lists(track_lists: &TrackLists) -> Result<()> {

    write_to_cache(TRACK_LISTS_CACHE, track_lists)?;

    Ok(())
}


pub fn read_track_lists() -> Result<TrackLists> {

    let track_lists = ron::de::from_reader(File::open(TRACK_LISTS_CACHE)?)?;

    Ok(track_lists)
}


fn write_to_cache(path: &str, content: &impl Serialize) -> Result<()> {

    fs::create_dir_all(Path::new(CACHE_PATH))?;
    let mut file = File::create(path)?;

    let content = ron::ser::to_string(content)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
