#[cfg(not(any(
    feature = "mysql-rustls",
    feature = "mysql-native-tls",
    feature = "sqlite-rustls",
    feature = "sqlite-native-tls",
    feature = "postgres-rustls",
    feature = "postgres-native-tls",
    feature = "redis"
)))]
fn get_storage() -> anyhow::Result<poem::session::MemoryStorage> {
    Ok(poem::session::MemoryStorage::new())
}

#[cfg(feature = "redis")]
pub async fn get_redis_storage() -> anyhow::Result<poem::session::RedisStorage> {
    let redis_url = env::var("REDIS_URL").context("REDIS_URL is not set")?;
    let client = redis::Client::open(redis_url)?;
    let con_manager = redis::aio::ConnectionManager::new(client).await?;
    poem::session::RedisStorage::new(con_manager)
}

#[cfg(all(
    not(feature = "redis"),
    any(
        feature = "mysql-rustls",
        feature = "mysql-native-tls",
        feature = "sqlite-rustls",
        feature = "sqlite-native-tls",
        feature = "postgres-rustls",
        feature = "postgres-native-tls"
    )
))]
pub async fn get_storage(
    db: sea_orm::DatabaseConnection,
) -> anyhow::Result<dxp_db_session::DbSessionStorage> {
    let storage = dxp_db_session::DbSessionStorage::new(db);
    storage.cleanup().await?;
    Ok(storage)
}
