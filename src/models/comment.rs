use std::{thread::sleep, time::Duration};

use chrono::{serde::ts_seconds, DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommentRaw {
    id: i32,
    message: String,
    author_id: i32,
    thread_id: i32,
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub message: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

#[server]
pub async fn get_by_thread_id(id: i32) -> Result<Vec<Comment>, ServerFnError> {
    use crate::database::get_db;
    use sqlx::query;
    match query!(
        "SELECT c.id, c.message, m.username, c.date
        FROM Comment as c
        INNER JOIN Member as m ON c.author_id=m.id
        WHERE c.thread_id = $1
        ORDER BY c.date ASC",
        id
    )
    .map(|x| Comment {
        id: x.id,
        message: x.message,
        author: x.username,
        date: x.date,
    })
    .fetch_all(get_db())
    .await
    {
        Ok(x) => Ok(x),
        Err(_x) => Err(ServerFnError::ServerError(
            "Something went wrong".to_string(),
        )),
    }
}

#[server(NewComment)]
pub async fn new_comment(
    message: String,
    author_id: i32,
    thread_id: i32,
) -> Result<(), ServerFnError> {
    println!("{} from {} on {}", message, author_id, thread_id);
    use crate::database::get_db;
    use sqlx::query;
    // sleep(Duration::from_secs(10));
    match query!(
        "INSERT INTO comment (message, author_id, thread_id) VALUES
        ($1, $2, $3)",
        message,
        author_id,
        thread_id
    )
    .execute(get_db())
    .await
    {
        Ok(x) => Ok(()),
        Err(err) => dbg!(Err(ServerFnError::ServerError(err.to_string()))),
    }
}
