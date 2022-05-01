use log::{error, info};
use std::env;

use dotenv::dotenv;

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();

    let series = fetch_series().await?;
    info!("{:?}", series);

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
