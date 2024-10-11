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
mod database;
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
pub use database::*;

#[cfg(feature = "redis")]
mod redis;
#[cfg(feature = "redis")]
pub use redis::*;

#[cfg(not(any(
    feature = "mysql-rustls",
    feature = "mysql-native-tls",
    feature = "sqlite-rustls",
    feature = "sqlite-native-tls",
    feature = "postgres-rustls",
    feature = "postgres-native-tls",
    feature = "redis"
)))]
mod memory;
#[cfg(not(any(
    feature = "mysql-rustls",
    feature = "mysql-native-tls",
    feature = "sqlite-rustls",
    feature = "sqlite-native-tls",
    feature = "postgres-rustls",
    feature = "postgres-native-tls",
    feature = "redis"
)))]
pub use memory::*;
