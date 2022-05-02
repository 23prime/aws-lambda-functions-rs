use std::env;

use log::{error, info};
use roxmltree::Document;
use sqlx::{Connection, PgConnection};

use atom::Atom;
use models::LatestEntry;

type BoxError = Box<dyn std::error::Error>;

pub mod atom;
pub mod error;
pub mod event;
pub mod line;
pub mod logger;
pub mod models;

pub async fn run() -> Result<(), BoxError> {
    let series = fetch_series().await?;
    let document = Document::parse(&series)?;
    let atom = Atom::parse(document);
    info!("Parse Atom: {:?}", atom);

    let feed = atom.feed.clone();
    let entry = feed.clone().latest_entry();
    info!("Latest Entry: {:?}", entry);
    let latest_entry = LatestEntry::new(feed.id, feed.title, entry.id);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::connect(&db_url).await?;

    let upsert_result = latest_entry.upsert(&mut conn).await?;
    info!("Upsert result: {:?}", upsert_result);

    let message = format!(
        "The new episode of {} has been released!\n\n{}\n{}",
        atom.feed.title, entry.title, entry.link
    );
    line::send(message).await?;

    return Ok(());
}

async fn fetch_series() -> Result<String, BoxError> {
    let entry_id = env::var("TYJ_SERIES_ID").expect("TYJ_SERIES_ID must be set");
    let url = format!("https://tonarinoyj.jp/atom/series/{}", entry_id);
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        error!("Failed to get series from tonarinoyj.jp: {:?}", response);
        return Err(Box::new(error::LambdaGeneralError::none()));
    }

    return Ok(response.text().await?);
}
