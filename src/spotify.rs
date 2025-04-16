use crate::{
    cache,
    Error
};
use anyhow::Result;
use spotify_rs::{
    auth::{ AuthCodePkceFlow, NoVerifier, Token },
    client::{ AuthCodePkceClient, Client },
    RedirectUrl,
};
use std::{
    collections::HashMap,
    io::{ self, BufRead, BufReader, Write },
    net::TcpListener,
    process::Command,
};
use url::Url;

pub type Spotify = Client<Token, AuthCodePkceFlow, NoVerifier>;

const CLIENT_ID: &str = "b91f8140e4014e0eaf126d0bb043f59c";
const LOOPBACK_ADDRESS: &str = "127.0.0.1:8080";
const REDIRECT_URL: &str = "http://127.0.0.1:8080";

pub async fn get_api() -> Result<Spotify> {
    let token = cache::read_token()?;
    let auth = AuthCodePkceFlow::new(CLIENT_ID, scopes());
    let mut spotify = Client::from_refresh_token(auth, false, token).await?;
    save_token(&mut spotify).await?;
    Ok(spotify)
}

pub async fn authenticate(cli_login: bool) -> Result<()> {
    let url = RedirectUrl::new(REDIRECT_URL.to_string())?;
    let auth = AuthCodePkceFlow::new(CLIENT_ID, scopes());
    let (client, url) = AuthCodePkceClient::new(auth, url, false);
    let (code, state) = if cli_login {
        prompt_cli(url.as_str())?
    } else {
        prompt_browser(url.as_str())?
    };
    let mut spotify = client.authenticate(code, state).await?;
    save_token(&mut spotify).await?;
    println!("Authentication successful");
    Ok(())
}

async fn save_token(spotify: &mut Spotify) -> Result<()> {
    spotify.request_refresh_token().await?;
    let token = spotify.refresh_token()
        .ok_or(Error::NoRefreshToken)?;
    cache::write_token(token)?;
    Ok(())
}

fn scopes() -> Vec<&'static str> {
    vec![
        "playlist-read-private",
        "playlist-modify-private",
        "user-library-modify",
        "user-read-currently-playing",
    ]
}

fn prompt_browser(url: &str) -> Result<(String, String)> {
    Command::new("zen-browser").arg(url).spawn()?;
    println!("URL opened in browser");
    let (mut stream, _) = TcpListener::bind(LOOPBACK_ADDRESS)?.accept()?;
    let mut response = String::new();
    BufReader::new(&stream).read_line(&mut response)?;
    stream.write_all("HTTP/1.1 200 OK\r\n\r\nDone".as_bytes())?;
    let queries = response.split(" ").skip(1).take(1).collect::<String>();
    let url = REDIRECT_URL.to_string() + &queries;
    let code_state = parse_queries(&url)?;
    Ok(code_state)
}

fn prompt_cli(url: &str) -> Result<(String, String)> {
    println!("Open url in browser:");
    println!("{url}");
    println!("");
    println!("Paste redirected url:");
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let code_state = parse_queries(&url)?;
    Ok(code_state)
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
