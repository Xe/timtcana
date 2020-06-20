use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::*;
use std::{thread, time};

/// Measures of temperature and humidity
#[derive(Debug, Clone)]
pub struct Measure {
    time: DateTime<Utc>,
    temperature: u8,
    humidity: u8,
}

fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    Ok(())
}
