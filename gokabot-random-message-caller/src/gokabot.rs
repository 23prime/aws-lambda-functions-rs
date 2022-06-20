use std::env;

use log::info;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Params {
    target_id: String,
}

impl Params {
    fn new(target_id: &str) -> Self {
        return Params {
            target_id: target_id.to_string(),
        };
    }
}

pub async fn call(target_id: &str) -> Result<Response, reqwest::Error> {
    let url = get_url();
    let params = Params::new(target_id);

    let result = Client::new()
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&params)
        .send()
        .await;

    info!("Call gokabot API [{}] => {:?}", target_id, result);
    return result;
}

fn get_url() -> String {
    let gokabot_base_url = env::var("GOKABOT_BASE_URI").expect("GOKABOT_BASE_URI must be set");
    return format!("{}/line/push/random", gokabot_base_url);
}
