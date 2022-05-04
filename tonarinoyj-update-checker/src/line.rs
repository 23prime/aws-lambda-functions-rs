use std::env;

use log::info;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::Serialize;

use crate::error;

#[derive(Debug, Clone)]
struct Config {
    line_channel_token: String,
    target_id: String,
}

#[derive(Debug, Serialize)]
struct Params {
    to: String,
    messages: Vec<Messages>,
}

#[derive(Debug, Serialize)]
struct Messages {
    r#type: String,
    text: String,
}

pub async fn send(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let params = get_params(&config, message);
    let response = post_message(&config, params).await?;

    info!("LINE response: {:?}", response);
    let status = response.status();

    if status.is_success() {
        return Ok(());
    } else {
        let body = response.text().await.unwrap();
        let err_str = format!("status = {:?}, body = {:?}", status, body);
        return Err(Box::new(error::LambdaGeneralError::new(err_str)));
    }
}

fn get_config() -> Config {
    let line_channel_token =
        env::var("LINE_CHANNEL_TOKEN").expect("LINE_CHANNEL_TOKEN must be set");
    let target_id = env::var("TARGET_ID").expect("TARGET_ID must be set");
    return Config {
        line_channel_token,
        target_id,
    };
}

fn get_params(config: &Config, message: String) -> Params {
    return Params {
        to: config.clone().target_id,
        messages: vec![Messages {
            r#type: "text".to_string(),
            text: message,
        }],
    };
}

async fn post_message(config: &Config, params: Params) -> Result<Response, reqwest::Error> {
    return Client::new()
        .post("https://api.line.me/v2/bot/message/push")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", config.line_channel_token),
        )
        .json(&params)
        .send()
        .await;
}
