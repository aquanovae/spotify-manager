use anyhow::Result;
use serde::{ Deserialize, Serialize };
use std::{
    fs,
    io::Write,
    os::unix::net::{ UnixListener, UnixStream },
};

pub const SOCKET_PATH: &str = "/tmp/spotify-manager/socket";

#[derive(Deserialize, Serialize)]
pub struct RefreshSignal;

pub struct Socket {
    listener: UnixListener,
}

impl Socket {
    pub fn new() -> Result<Socket> {
        let listener = UnixListener::bind(SOCKET_PATH)?;
        listener.set_nonblocking(true)?;
        Ok(Socket {
            listener,
        })
    }

    pub fn poll_signal(&self) -> bool {
        if let Ok((mut stream, _)) = self.listener.accept() {
            ron::de::from_reader::<_, RefreshSignal>(stream).is_ok()
        }
        else {
            false
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        fs::remove_file(SOCKET_PATH).unwrap()
    }
}

pub fn send_signal() -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    stream.write_all(
        ron::ser::to_string(&RefreshSignal)?.as_bytes()
    )?;
    Ok(())
}
