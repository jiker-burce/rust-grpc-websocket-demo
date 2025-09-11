use redis::Client;
use std::env;

pub async fn create_redis_client() -> Result<Client, redis::RedisError> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    Client::open(redis_url)
}
