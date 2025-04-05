use anyhow::Result;

use spotify_rs::{
    auth::{ AuthCodePkceFlow, NoVerifier, Token },
    client::{ AuthCodePkceClient, Client },
    RedirectUrl,
};

const CLIENT_ID: &str = "b91f8140e4014e0eaf126d0bb043f59c";
const REDIRECT_URL: &str = "http://127.0.0.1:8080";

pub fn authenticate() -> Result<Client<Token, AuthCodePkceFlow, NoVerifier>> {
    let url = RedirectUrl::new(REDIRECT_URL.to_owned())?;
    let scopes = vec![
        "playlist-read-private",
        "playlist-modify-private",
        "user-library-modify",
        "user-read-currently-playing",
    ];
    let auth = AuthCodePkceFlow::new(CLIENT_ID, scopes);
    let (client, url) = AuthCodePkceClient::new(auth, url, true);
    webbrowser::open(url.as_str())?;
    todo!()
}

/*
async fn spotify_api(
    url: &str
) -> Result<Client<Token, AuthCodePkceFlow, NoVerifier>> {
    let url = RedirectUrl::new(url.to_owned())?;
}
*/
