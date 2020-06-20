use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::*;
use rusqlite::Connection;
use std::{thread, time, path::PathBuf};
use structopt::StructOpt;

/// Measures of temperature and humidity
#[derive(Debug, Clone)]
pub struct Measure {
    time: DateTime<Utc>,
    temperature: u8,
    humidity: u8,
}

#[derive(StructOpt, Debug)]
struct Cmd {
    /// Sqlite Database location
    pub db_loc: PathBuf,

    /// Port to listen on
    #[structopt(long, env = "PORT", default_value = "9001")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    let cmd = Cmd::from_args();

    println!("starting: {:?}", cmd);

    let db = Connection::open(cmd.db_loc)?;

    Ok(())
}
