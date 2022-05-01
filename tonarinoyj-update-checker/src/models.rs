use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LatestEntry {
    pub series_id: String,
    pub series_name: String,
    pub entry_id: String,
}
