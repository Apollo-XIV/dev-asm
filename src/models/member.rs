use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    id: i64,
    pub username: String,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

// impl Member {
//     fn new() -> Self {
//         Member {}
//     }
// }
