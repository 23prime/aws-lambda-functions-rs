use log::info;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rusoto_core::region::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};
use serde::Serialize;

pub mod error;
pub mod event;

pub async fn run(event: event::Event) -> Result<(), Box<dyn std::error::Error>> {
    let message = format!(
        "[Date]\n{}\n\n[Subject]\n{}\n\n[Message]\n{}",
        event.Records[0].Sns.Timestamp, event.Records[0].Sns.Subject, event.Records[0].Sns.Message
    );

    info!("[run] message = {:?}", message);

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
    info!("[send_to_line] Send a message to LINE.");

    let target_id = get_ssm_param("gokabot.LINE_CHANNEL_TOKEN").await?;
    let line_channel_token = get_ssm_param("gokabot.MY_USER_ID").await?;

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

    info!("[send_to_line] response = {:?}", response);

    let status = response.status();

    if status.is_success() {
        return Ok(());
    } else {
        let body = response.text().unwrap();
        let err_str = format!("status = {:?}, body = {:?}", status, body);
        return Err(Box::new(error::from(err_str)));
    }
}

async fn get_ssm_param(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    info!("[get_ssm_param] Get SSM Parameter value of [{:?}]", key);

    let client: SsmClient = SsmClient::new(Region::ApNortheast1);

    let result = client
        .get_parameter(GetParameterRequest {
            name: key.to_string(),
            with_decryption: Some(true),
        })
        .await?
        .parameter
        .unwrap()
        .value
        .unwrap();

    info!("[get_ssm_param] Done.");
    return Ok(result);
}
