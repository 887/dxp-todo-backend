use anyhow::Context;
use redis_pool::{RedisPool, SingleRedisPool};

pub type SessionPoolType = axum_session_redispool::SessionRedisPool;

pub type SessionType = axum_session::Session<SessionPoolType>;

pub async fn get_pool() -> anyhow::Result<SessionPoolType> {
    //https://github.com/AscendingCreations/AxumSession/blob/main/examples/redis/src/main.rs

    let redis_url = std::env::var("REDIS_URL").context("REDIS_URL is not set")?;
    let client = redis::Client::open(redis_url)?;

    Ok(RedisPool::from(client));
}
