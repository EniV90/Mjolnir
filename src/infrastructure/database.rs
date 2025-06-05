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

// test db
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
}

pub async fn create_user(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
    INSERT INTO users(username)
    VALUES ($1)
    RETURNING id, username
    "#,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
