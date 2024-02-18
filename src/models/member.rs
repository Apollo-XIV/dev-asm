use chrono::{serde::ts_seconds, DateTime, Utc};
use leptos::logging::log;
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
    use sqlx::query_as;
    query_as!(Member,
        "SELECT m.id, m.gh_id, m.av_url, m.username, m.date
         FROM Member as m
         WHERE m.id=$1",
        id
    )
    .fetch_one(get_db())
    .await
    .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

impl Member {
    // check if user already exists, if yes update info, else create new user
    #[cfg(feature = "ssr")]
    pub async fn patch(gh_id: i32, username: String, av_url: String) -> Result<Self, String> {
        use sqlx::{query_as, query};

        // start db transaction
        let mut tx = crate::database::get_db()
            .begin()
            .await
            .map_err(|_| "Could not connect to the database".to_string())?;
        
        // check if user with gh id or email exists
        let new_user = match query!("SELECT id FROM Member WHERE gh_id = $1", gh_id)
            .map(|row| row.id)
            .fetch_one(&mut *tx)
            .await
        {
            // if they exist, update with new avatar url
            Ok(id) => query_as!(Member, 
                "UPDATE Member SET av_url = $1 WHERE id = $2 RETURNING id, gh_id, av_url, username, date", 
                av_url, 
                id)
            .fetch_one(&mut *tx)
            .await,
            // otherwise, create a new user record
            Err(_) => query_as!(Member, 
                "INSERT INTO Member(gh_id, username, av_url) VALUES ($1, $2, $3) RETURNING id, gh_id, av_url, username, date",
                gh_id,
                username,
                av_url)
            .fetch_one(&mut *tx)
            .await
        }
        .map_err(|_| "Couldn't create new record".to_string())?;
        
        tx.commit()
            .await
            .map_err(|_| "Couldn't commit the transaction".to_string())?;
        Ok(new_user)
    }

    fn null() {
        ()
    }
}
