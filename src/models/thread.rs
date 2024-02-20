use chrono::{serde::ts_seconds, DateTime, Utc};
use leptos::logging::log;
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
    query_as!(
        ThreadRaw,
        "SELECT id, title, date, author_id FROM Thread ORDER BY date DESC;"
    )
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
        "SELECT t.id, t.title, t.date, m.username
         FROM Thread as t
         INNER JOIN Member as m ON t.author_id=m.id
         ORDER BY t.date
         DESC"
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
pub async fn new_thread(title: String, message: String) -> Result<(), ServerFnError> {
    use crate::auth::auth;
    use crate::auth::Claims;
    use crate::database::get_db;
    use crate::models::member::Member;
    use actix_web::HttpRequest;
    use leptos::ServerFnError::*;
    use leptos_actix::extract;
    use sqlx::query;
    // log!("{}{}{}", title, message, author_id);
    // extract authed user or return error;
    let user: Member = extract(auth).await??;
    // log!("{:?}", user);
    let mut tx = get_db().begin().await?;
    let new_thread_id = query!(
        "INSERT INTO thread (title, author_id)
                 VALUES ($1, $2) RETURNING id;",
        title,
        user.id,
    )
    .map(|row| row.id)
    .fetch_one(&mut *tx)
    .await?;
    query!(
        "INSERT INTO Comment (message, author_id, thread_id)
        VALUES ($1, $2, $3)",
        message,
        user.id,
        new_thread_id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    leptos_actix::redirect(&format!("/forum/{new_thread_id:?}"));
    Ok(())
}
