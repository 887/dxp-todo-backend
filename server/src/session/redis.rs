#[cfg(feature = "redis")]
pub async fn get_redis_storage() -> anyhow::Result<poem::session::RedisStorage> {
    let redis_url = env::var("REDIS_URL").context("REDIS_URL is not set")?;
    let client = redis::Client::open(redis_url)?;
    let con_manager = redis::aio::ConnectionManager::new(client).await?;
    poem::session::RedisStorage::new(con_manager)
}
