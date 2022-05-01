use std::env;

use dotenv::dotenv;
use log::{error, info};
use roxmltree::Document;

use types::Feed;

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;
pub mod types;

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();

    let series = fetch_series().await?;
    let document = Document::parse(&series)?;
    let node = document.root_element();

    let feed = Feed::parse(node);
    info!("{:?}", feed);

    let latest_entry = feed.latest_entry();
    info!("Latest Entry: {:?}", latest_entry);

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
