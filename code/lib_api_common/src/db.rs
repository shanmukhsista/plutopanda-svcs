use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPoolOptions};

    pub type DatabasePool = Pool<Postgres>;

const DB_MAX_CONNECTIONS: u32 = 10;
const DB_CONNECTION_TIMEOUT: u64 = 3;

pub async fn get_database_pool() -> Result<DatabasePool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    // set up connection pool
    tracing::trace!("Connecting to database. {}", db_url);
    PgPoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(DB_CONNECTION_TIMEOUT))
        .connect(&db_url).await
}

