use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgConnection;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LatestEntry {
    pub series_id: String,
    pub series_name: String,
    pub entry_id: String,
}

impl LatestEntry {
    pub fn new(series_id: String, series_name: String, entry_id: String) -> Self {
        return LatestEntry {
            series_id,
            series_name,
            entry_id,
        };
    }

    pub async fn select_by_series_id(
        conn: &mut PgConnection,
        series_id: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        return sqlx::query_as::<_, Self>(
            "
            SELECT *
            FROM tonarinoyj_update_checker.latest_entries
            WHERE series_id = $1",
        )
        .bind(series_id)
        .fetch_optional(conn)
        .await;
    }

    pub async fn update(&self, conn: &mut PgConnection) -> Result<Vec<Self>, sqlx::Error> {
        return sqlx::query_as::<_, Self>(
            "
            UPDATE tonarinoyj_update_checker.latest_entries
            SET entry_id = $1, updated_at = CURRENT_TIMESTAMP
            WHERE series_id = $2",
        )
        .bind(&self.entry_id)
        .bind(&self.series_id)
        .fetch_all(conn)
        .await;
    }

    pub async fn create(&self, conn: &mut PgConnection) -> Result<Vec<Self>, sqlx::Error> {
        return sqlx::query_as::<_, Self>(
            "
            INSERT INTO tonarinoyj_update_checker.latest_entries (series_id, series_name, entry_id)
            VALUES ($1, $2, $3)",
        )
        .bind(&self.series_id)
        .bind(&self.series_name)
        .bind(&self.entry_id)
        .fetch_all(conn)
        .await;
    }
}
