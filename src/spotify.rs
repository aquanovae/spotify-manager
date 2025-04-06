use crate::Error;

use anyhow::Result;
use spotify_rs::{
    auth::{ AuthCodePkceFlow, NoVerifier, Token },
    client::{ AuthCodePkceClient, Client },
    RedirectUrl,
};
use std::{
    collections::HashMap,
    fs::{ self, File },
    io::{ self, BufRead, BufReader, Read, Write },
    net::TcpListener,
    path::Path,
    process::Command,
    time::Duration,
};
use url::Url;

const CLIENT_ID: &str = "b91f8140e4014e0eaf126d0bb043f59c";
const LOOPBACK_ADDRESS: &str = "127.0.0.1:8080";
const CACHE_FILE: &str = "/var/cache/spotify-token/token.txt";

pub async fn authenticate(server_mode: bool) -> Result<()> {
    let url = RedirectUrl::new(String::from("http://") + LOOPBACK_ADDRESS)?;
    let scopes = vec![
        "playlist-read-private",
        "playlist-modify-private",
        "user-library-modify",
        "user-read-currently-playing",
    ];
    let auth = AuthCodePkceFlow::new(CLIENT_ID, scopes);
    let (client, url) = AuthCodePkceClient::new(auth, url, true);
    let (code, state) = if server_mode {
        prompt_cli(url.as_str())?
    } else {
        prompt_browser(url.as_str())?
    };
    let spotify = client.authenticate(code, state).await?;
    let refresh_token = spotify.refresh_token().ok_or(Error::NoRefreshToken)?;
    let cache_path = Path::new(CACHE_FILE).parent().unwrap();
    fs::create_dir_all(cache_path)?;
    let cache_file = File::create(CACHE_FILE)?;
    Ok(())
}

fn prompt_browser(url: &str) -> Result<(String, String)> {
    Command::new("zen-browser").arg(url).spawn()?;
    let (mut stream, _) = TcpListener::bind(LOOPBACK_ADDRESS)?.accept()?;
    let mut response = String::new();
    BufReader::new(&stream).read_line(&mut response)?;
    stream.write_all("HTTP/1.1 200 OK\r\n\r\nDone".as_bytes())?;
    let queries = response.split(" ").skip(1).take(1).collect::<String>();
    let url = String::from("http://") + LOOPBACK_ADDRESS + &queries;
    parse_queries(&url)
}

fn prompt_cli(url: &str) -> Result<(String, String)> {
    println!("Open url in browser:");
    println!("{url}");
    println!("");
    println!("Paste redirected url:");
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    parse_queries(&url)
}

fn parse_queries(url: &str) -> Result<(String, String)> {
    let mut queries = HashMap::<String, String>::new();
    Url::parse(url)?
        .query_pairs()
        .for_each(|query| {
            let (query, value) = query;
            queries.insert(query.into_owned(), value.into_owned());
        });
    let code = queries
        .remove("code")
        .ok_or(Error::ParseAuthResponse)?;
    let state = queries
        .remove("state")
        .ok_or(Error::ParseAuthResponse)?;
    Ok((code, state))
}
