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
    let fetched_latest_entry = feed.clone().latest_entry();
    info!("Fetched latest entry: {:?}", fetched_latest_entry);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::connect(&db_url).await?;

    let recorded_latest_entry = LatestEntry::select_by_series_id(&mut conn, &feed.id).await?;
    info!("Recorded latest entry: {:?}", recorded_latest_entry);
    let latest_entry = LatestEntry::new(feed.id, feed.title, fetched_latest_entry.clone().id);

    if recorded_latest_entry.is_none() {
        latest_entry.create(&mut conn).await?;
        info!("Create latest entry.");
        return Ok(());
    }

    if fetched_latest_entry.id == recorded_latest_entry.unwrap().entry_id {
        info!("Not updated: latest entry ID: {}", fetched_latest_entry.id);
        return Ok(());
    }

    latest_entry.update(&mut conn).await?;

    let message = format!(
        "The new episode of {} has been released!\n\n{}\n{}",
        atom.feed.title, fetched_latest_entry.title, fetched_latest_entry.link
    );
    line::send(message).await?;

    return Ok(());
}

async fn fetch_series() -> Result<String, BoxError> {
    let series_id = env::var("TYJ_SERIES_ID").expect("TYJ_SERIES_ID must be set");
    let url = format!("https://tonarinoyj.jp/atom/series/{}", series_id);
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        error!("Failed to get series from tonarinoyj.jp: {:?}", response);
        return Err(Box::new(error::LambdaGeneralError::none()));
    }

    return Ok(response.text().await?);
}
