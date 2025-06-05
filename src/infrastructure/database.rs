use crate::application::config::Config;
use sqlx::{PgPool, Row, pool};

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(&config.database_url).await?;

    //testing the connection
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    tracing::info!(
        "Database connected successfully. Test query result: {}",
        row.0
    );
    Ok(pool)
}

