CREATE TABLE tonarinoyj_update_checker.latest_entries (
    series_id VARCHAR(255) PRIMARY KEY,
    series_name VARCHAR(255),
    entry_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
