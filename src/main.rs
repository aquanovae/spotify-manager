mod spotify;

use anyhow::Result;

use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Authenticate user
    Auth {
        #[arg(short, default_value_t = false)]
        server_login: bool,
    }
}

fn main() -> Result<()> {
    spotify::authenticate()?;
    Ok(())
}
