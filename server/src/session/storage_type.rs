#[cfg(feature = "redis")]
pub type SessionStorageType = poem::session::RedisStorage;
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
pub type SessionStorageType = dxp_db_session::DbSessionStorage;
#[cfg(not(any(
    feature = "mysql-rustls",
    feature = "mysql-native-tls",
    feature = "sqlite-rustls",
    feature = "sqlite-native-tls",
    feature = "postgres-rustls",
    feature = "postgres-native-tls",
    feature = "redis"
)))]
pub type SessionStorageType = poem::session::MemoryStorage;
