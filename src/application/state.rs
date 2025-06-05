use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: RedisPool,
}

impl AppState {
    pub fn new(db_pool: PgPool, redis_pool: RedisPool) -> Self {
        Self {
            db_pool,
            redis_pool,
        }
    }
}
