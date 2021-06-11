use std::env;

use jsonxf::pretty_print;
use log::info;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;

pub mod error;
pub mod event;
pub mod logger;

pub async fn run(event: event::Event) -> Result<(), Box<dyn std::error::Error>> {
    let sns_message = format_sns_message(&event.Records[0].Sns.Message);

    let message = format!(
        "# From AWS/SNS\n## Date\n{}\n\n## Subject\n{}\n\n## Message\n```\n{}",
        event.Records[0].Sns.Timestamp, event.Records[0].Sns.Subject, sns_message
    );

    info!("message = {:?}", message);

    return send_to_msteams(message).await;
}

fn format_sns_message(message: &str) -> String {
    if !message.contains("{") {
        return message.to_string();
    }

    return match pretty_print(message) {
        Ok(m) => m,
        _ => message.to_string(),
    };
}

#[derive(Debug, Serialize)]
struct Params {
    text: String,
}

impl Params {
    fn new(text: String) -> Params {
        return Params { text: text };
    }
}

async fn send_to_msteams(message: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Send a message to Microsoft Teams Incoming Webhook.");

    let webhook_url = env::var("WEBHOOK_URL").unwrap();
    let params: Params = Params::new(message);

    let client = Client::new();
    let response = client
        .post(webhook_url)
        .header(CONTENT_TYPE, "application/json")
        .json(&params)
        .send()?;

    info!("response = {:?}", response);

    let status = response.status();

    if status.is_success() {
        return Ok(());
    } else {
        let body = response.text().unwrap();
        let err_str = format!("status = {:?}, body = {:?}", status, body);
        return Err(Box::new(error::LambdaGeneralError::new(err_str)));
    }
}
