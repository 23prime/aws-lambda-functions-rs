use std::collections::HashMap;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CloudWatchScheduledEvent {
    id: String,
    #[serde(rename(deserialize = "detail-type"))]
    detail_type: String,
    source: String,
    account: String,
    time: String,
    region: String,
    resources: Vec<String>,
    detail: HashMap<String, String>,
}
