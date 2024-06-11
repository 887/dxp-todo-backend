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
