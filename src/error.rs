#[derive(Debug, thiserror::Error)]
pub enum Error {

    #[error("No refresh token found")]
    NoRefreshToken,

    #[error("No track playing")]
    NoTrackPlaying,

    #[error("Error parsing spotify authentication response")]
    ParseAuthResponse,

    #[error("Playlist not fetched")]
    PlaylistNotFetched,

    #[error("Could not connect to spotify API aftre too many attempts")]
    TooManyRetries,
}

