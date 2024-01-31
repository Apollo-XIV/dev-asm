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
    use sqlx::query_as;
    query_as!(ThreadRaw, "SELECT id, title, date, author_id FROM Thread;")
        .fetch_all(database::get_db())
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server]
pub async fn get_by_id(id: i32) -> Result<Thread, ServerFnError> {
    use crate::database::get_db;
    use sqlx::query;
    query!(
        "SELECT t.id, t.title, t.date, m.username
         FROM Thread as t
         INNER JOIN Member as m ON t.author_id=m.id
         WHERE t.id=$1",
        id
    )
    .map(|x| Thread {
        id: x.id,
        title: x.title,
        date: x.date,
        author: x.username,
    })
    .fetch_one(get_db())
    .await
    .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server]
pub async fn get_all() -> Result<Vec<Thread>, ServerFnError> {
    use crate::database;
    use sqlx::{query, query_as};
    query!(
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
    .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server(NewThread)]
/// Creates a new thread, along with an initial comment
pub async fn new_thread(
    title: String,
    message: String,
    author_id: i32,
) -> Result<(), ServerFnError> {
    use crate::database::get_db;
    use sqlx::query;
    match query!(
        "INSERT INTO thread (title, author_id)
         VALUES ($1, $2)",
        title,
        author_id
    )
    .execute(get_db())
    .await
    {
        Ok(x) => Ok(()),
        Err(err) => Err(ServerFnError::ServerError(err.to_string())),
    }
}
