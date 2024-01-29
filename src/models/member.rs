use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Member {
    id: i64,
    username: String,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}
