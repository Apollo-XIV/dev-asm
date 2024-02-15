use chrono::{serde::ts_seconds, DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: i32,
    pub gh_id: i32,
    pub av_url: Option<String>,
    pub username: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

#[server]
async fn get_by_id(id: i32) -> Result<Member, ServerFnError> {
    use crate::database::get_db;
    use sqlx::query;
    query!(
        "SELECT m.id, m.gh_id, m.av_url, m.username, m.joined
         FROM Member as m
         WHERE m.id=$1",
        id
    )
    .map(|m| Member {
        id: m.id,
        gh_id: m.gh_id,
        av_url: m.av_url,
        date: m.joined,
        username: m.username,
    })
    .fetch_one(get_db())
    .await
    .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

impl Member {
    // check if user already exists, if yes update info, else create new user
    #[cfg(feature = "ssr")]
    pub async fn patch(gh_id: i32, username: String, av_url: String) -> Result<Self, String> {
        use futures_util::FutureExt;

        let tx = crate::database::get_db()
            .begin()
            .await
            .map_err(|_| "Could not connect to the database".to_string())?;
        // start db transaction
        // check if user with gh id or email exists
        // if they exist, update with new avatar url
        // otherwise, create a new user record
        tx.commit();
        Ok(Member {
            id: 0,
            gh_id: 0,
            av_url: None,
            username: "none".into(),
            date: chrono::offset::Utc::now(),
        })
    }

    fn null() {
        ()
    }
}
