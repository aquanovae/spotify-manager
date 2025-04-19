use crate::ipc;
use anyhow::Result;
use std::{
    io::Write,
    os::unix::net::UnixStream,
};

pub fn to_destination() -> Result<()> {
    ipc::send_signal()?;
    Ok(())
}
