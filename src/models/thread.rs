use chrono::{serde::ts_seconds, DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadRaw {
    pub id: i32,
    pub title: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub author_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub author: String,
}

#[server]
pub async fn get_all_raw() -> Result<Vec<ThreadRaw>, ServerFnError> {
    use crate::database;
    use sqlx::{query, query_as};
    match query_as!(
        ThreadRaw,
        r#"SELECT id, title, date, author_id FROM Thread;"#
    )
    .fetch_all(database::get_db())
    .await
    {
        Ok(x) => Ok(x),
        Err(x) => Err(ServerFnError::ServerError(
            "something went wrong connecting to the database".to_string(),
        )),
    }
}

#[server]
pub async fn get_all() -> Result<Vec<Thread>, ServerFnError> {
    use crate::database;
    use sqlx::{query, query_as};
    match query!(
        "SELECT Thread.id, Thread.title, Thread.date, Member.username
        FROM Thread
        INNER JOIN Member ON Thread.author_id=Member.id"
    )
    .map(|x| Thread {
        id: x.id,
        title: x.title,
        date: x.date,
        author: x.username,
    })
    .fetch_all(database::get_db())
    .await
    {
        Ok(x) => Ok(x),
        Err(x) => Err(ServerFnError::ServerError(
            "something went wrong connecting to the database".to_string(),
        )),
    }
}
