use sqlx::{pool, PgPool, Row};
use crate::application::config::Config;

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
  let pool = PgPool::connect(&config.database_url).await?;


  //testing the connection
  let row: (i64,) = sqlx::query_as("SELECT $1")
  .bind(150_i64)
  .fetch_one(&pool)
  .await?;

  tracing::info!("Database connected successfully. Test query result: {}", row.0);
  Ok(pool)
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
  pub id: uuid::Uuid,
  pub username: String,
  // pub email: String

}

// pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
//   let user = sqlx::query_as!(
//     User,
//     "SELECT id, username, email FROM users WHERE email = $1",
//     email
//   )
//   .fetch_optional(pool)
//   .await?;

//   Ok(user)
// }

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