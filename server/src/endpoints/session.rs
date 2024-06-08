use std::env;

use anyhow::Context;
use anyhow::Result;
use poem::{
    session::{CookieConfig, ServerSession, SessionStorage},
    web::cookie::CookieKey,
};

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use sea_orm::DatabaseConnection;

//Result<_, Box<impl std::error::Error>>
pub fn get_sever_session<S>(storage: S) -> Result<ServerSession<S>>
where
    S: SessionStorage,
{
    let cookie_key = env::var("COOKIE_KEY").context("COOKIE_KEY is not set")?;

    let cookie_key_bytes =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
            .decode(cookie_key)
            .context("COOKIE_KEY not base64")?;

    let cookie_key = CookieKey::from(&cookie_key_bytes);

    Ok(ServerSession::new(
        CookieConfig::signed(cookie_key),
        storage,
    ))
}

#[cfg(not(any(
    feature = "mysql-rustls",
    feature = "mysql-native-tls",
    feature = "sqlite-rustls",
    feature = "sqlite-native-tls",
    feature = "postgres-rustls",
    feature = "postgres-native-tls",
    feature = "redis"
)))]
fn get_storage() -> Result<impl SessionStorage + Clone> {
    Ok(poem::session::MemoryStorage::new())
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
pub async fn get_db_storage(db: DatabaseConnection) -> Result<impl SessionStorage + Clone> {
    let storage = dxp_db_session::DbSessionStorage::new(db);
    storage.cleanup().await?;
    Ok(storage)
}

#[cfg(feature = "redis")]
pub async fn get_redis_storage(db: DatabaseConnection) -> Result<impl SessionStorage + Clone> {
    let redis_url = env::var("REDIS_URL").context("REDIS_URL is not set")?;
    let client = redis::Client::open(redis_url)?;
    let con_manager = redis::aio::ConnectionManager::new(client).await?;
    poem::session::RedisStorage::new(con_manager)
}
