use chrono::{DateTime, Utc};
use leptos::*;

const SECONDS_P_YEAR: i64 = 31_536_000;
const SECONDS_P_MONTH: i64 = 2_592_000;
const SECONDS_P_DAY: i64 = 86_400;
const SECONDS_P_HOUR: i64 = 3_600;
const SECONDS_P_MINUTE: i64 = 60;

pub fn time_since(dt: DateTime<Utc>) -> String {
    let since = chrono::offset::Utc::now().timestamp() - dt.timestamp();
    match since {
        s if s > SECONDS_P_YEAR => format!("{}y", s / SECONDS_P_YEAR),
        s if s > SECONDS_P_MONTH => format!("{}mo.", s / SECONDS_P_MONTH),
        s if s > SECONDS_P_DAY => format!("{}d", s / SECONDS_P_DAY),
        s if s > SECONDS_P_HOUR => format!("{}h", s / SECONDS_P_HOUR),
        s if s > SECONDS_P_MINUTE => format!("{}m", s / SECONDS_P_MINUTE),
        s => format!("{}s", s),
    }
}

#[server(GetClientId, "/api", "Cbor", "get_client_id")]
pub async fn get_client_id() -> Result<String, ServerFnError> {
    Ok(dbg!(crate::CLIENT_ID.to_owned()))
}
