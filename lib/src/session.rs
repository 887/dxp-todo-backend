use std::env;

use anyhow::Context;
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
pub fn get_sever_session<S>(storage: S) -> Result<ServerSession<S>, anyhow::Error>
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
fn get_storage() -> Result<impl SessionStorage, anyhow::Error> {
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
pub async fn get_db_storage(db: DatabaseConnection) -> Result<impl SessionStorage, anyhow::Error> {
    let storage = dbsession::DbSessionStorage::new(db);
    storage.cleanup().await?;
    Ok(storage)
}
