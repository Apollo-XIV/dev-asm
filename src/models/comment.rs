use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    id: i32,
    title: Option<String>,
    message: String,
    author_id: i32,
    thread_id: i32,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}
