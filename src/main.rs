use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::*;
use serde::{Deserialize, Serialize};
use std::process::Command;
use structopt::StructOpt;
use warp::Filter;

/// Measures of temperature and humidity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure {
    time: DateTime<Utc>,
    temperature: f32,
    humidity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    why: String,
    kind: String,
}

#[derive(StructOpt, Debug)]
struct Cmd {
    /// Sqlite Database location
    pub db_loc: String,

    /// Port to listen on
    #[structopt(long, env = "PORT", default_value = "9001")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    let cmd = Cmd::from_args();

    println!("starting: {:?}", cmd);
    println!("{:?}", get_newest_temps(cmd.db_loc.clone())?);

    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        info!(
            "{:?} {} {} {}",
            info.elapsed(),
            info.method(),
            info.path(),
            info.status()
        );
    });

    let current_temp = warp::path("now")
        .map(|| match get_newest_temps("data.db".into()) {
            Ok(temp) => warp::reply::json(&temp),
            Err(why) => warp::reply::json(&Error {
                why: format!("{:?}", why),
                kind: "temperature error".into(),
            }),
        })
        .with(log);

    warp::serve(current_temp)
        .run(([0, 0, 0, 0], cmd.port))
        .await;

    Ok(())
}

fn get_newest_temps(db_fname: String) -> Result<Measure> {
    let output = Command::new("sqlite3")
        .arg(db_fname)
        .arg("select * from temp order by rowid desc limit 1;")
        .output()?;

    let mut stdout: String = String::from_utf8(output.stdout)?;
    stdout = stdout.trim().into();
    let cols: Vec<&str> = stdout.split("|").collect();
    let time = cols[0];
    let temp = cols[1];
    let hum = cols[2];

    let time = time.parse::<i64>()?;
    let naive = NaiveDateTime::from_timestamp(time, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    Ok(Measure {
        time: datetime,
        temperature: temp.parse::<f32>()?,
        humidity: hum.parse::<f32>()?,
    })
}
