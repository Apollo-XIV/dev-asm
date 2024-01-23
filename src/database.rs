use sqlx::{PgPool, Pool, postgres::PgPoolOptions};

#[derive(Debug)]
struct Thread {
	id: i32,
	name: String,
	test: i32
}

async fn get_threads(pool: &PgPool) -> Result<Vec<Thread>,sqlx::Error> {
	let threads = sqlx::query_as!(
		Thread,
		r#"SELECT id, name, test FROM threads;"#
	)
	.
	.fetch_all(pool)
	.await?;
	Ok(threads)
}

static DB: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}

pub async fn init_db() -> Result<(), sqlx::Pool<sqlx::Postgres>> {
    DB.set(create_pool().await)
}

pub fn get_db<'a>() -> &'a sqlx::PgPool {
    DB.get().expect("database unitialized")
}
