use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};

pub type RedisConnection = deadpool_redis::Connection;

pub async fn create_pool(redis_url: &str) -> Result<RedisPool, Box<dyn std::error::Error>> {
  let cfg = RedisConfig::from_url(redis_url);
  let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

  let mut connect = pool.get().await?;
  let pong: String = redis::cmd("PING").query_async(&mut connect).await?;
  tracing::info!("Redis connected successfully. Ping response: {}",pong);

  Ok(pool)
}