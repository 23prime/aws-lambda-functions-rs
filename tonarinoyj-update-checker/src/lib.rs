use std::env;

use dotenv::dotenv;
use log::{error, info};
use roxmltree::Document;
use sqlx::{Connection, PgConnection, Row};

use models::LatestEntry;
use types::{Entry, Feed};

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;
pub mod models;
pub mod types;

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();

    let series = fetch_series().await?;
    let document = Document::parse(&series)?;
    let node = document.root_element();

    let feed = Feed::parse(node);
    info!("{:?}", feed);

    let latest_entry = feed.clone().latest_entry();
    info!("Latest Entry: {:?}", latest_entry);

    let upsert_result = upsert_latest_entry(feed, latest_entry).await?;
    info!("{:?}", upsert_result);

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

async fn upsert_latest_entry(
    feed: Feed,
    latest_entry: Entry,
) -> Result<Vec<LatestEntry>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::connect(&db_url).await?;

    if series_exists(&mut conn, &feed.id).await? {
        return update_latest_entry(&mut conn, feed, latest_entry).await;
    } else {
        return create_latest_entry(&mut conn, feed, latest_entry).await;
    }
}

async fn series_exists(conn: &mut PgConnection, series_id: &str) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query(
        "
        SELECT COUNT(*)
        FROM tonarinoyj_update_checker.latest_entries
        WHERE series_id = $1",
    )
    .bind(series_id)
    .fetch_one(conn)
    .await?
    .get("count");

    return Ok(count > 0);
}

async fn update_latest_entry(
    conn: &mut PgConnection,
    feed: Feed,
    latest_entry: Entry,
) -> Result<Vec<LatestEntry>, sqlx::Error> {
    return sqlx::query_as::<_, LatestEntry>(
        "
        UPDATE tonarinoyj_update_checker.latest_entries
        SET entry_id = $1, updated_at = CURRENT_TIMESTAMP
        WHERE series_id = $2",
    )
    .bind(&latest_entry.id)
    .bind(&feed.id)
    .fetch_all(conn)
    .await;
}

async fn create_latest_entry(
    conn: &mut PgConnection,
    feed: Feed,
    latest_entry: Entry,
) -> Result<Vec<LatestEntry>, sqlx::Error> {
    return sqlx::query_as::<_, LatestEntry>(
        "
        INSERT INTO tonarinoyj_update_checker.latest_entries (series_id, series_name, entry_id)
        VALUES ($1, $2, $3)"
    )
    .bind(&feed.id)
    .bind(&feed.title)
    .bind(&latest_entry.id)
    .fetch_all(conn)
    .await;
}
