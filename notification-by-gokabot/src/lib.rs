use std::env;

use log::info;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;

pub mod error;
pub mod event;
pub mod logger;

pub async fn run(event: event::Event) -> Result<(), Box<dyn std::error::Error>> {
    let message = format!(
        "[Date]\n{}\n\n[Subject]\n{}\n\n[Message]\n{}",
        event.Records[0].Sns.Timestamp, event.Records[0].Sns.Subject, event.Records[0].Sns.Message
    );

    info!("message = {:?}", message);

    return send_to_line(message).await;
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

async fn send_to_line(message: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Send a message to LINE.");

    let line_channel_token = env::var("LINE_CHANNEL_TOKEN").unwrap();
    let target_id = env::var("MY_USER_ID").unwrap();

    let params: Params = Params {
        to: target_id,
        messages: vec![Messages {
            r#type: "text".to_string(),
            text: message,
        }],
    };

    let client = Client::new();
    let response = client
        .post("https://api.line.me/v2/bot/message/push")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", line_channel_token))
        .json(&params)
        .send()?;

    info!("response = {:?}", response);

    let status = response.status();

    if status.is_success() {
        return Ok(());
    } else {
        let body = response.text().unwrap();
        let err_str = format!("status = {:?}, body = {:?}", status, body);
        return Err(Box::new(error::GokabotLambdaError::new(err_str)));
    }
}
