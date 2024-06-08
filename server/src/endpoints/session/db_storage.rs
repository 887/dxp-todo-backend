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
pub async fn get_db_storage(
    db: sea_orm::DatabaseConnection,
) -> anyhow::Result<dxp_db_session::DbSessionStorage> {
    let storage = dxp_db_session::DbSessionStorage::new(db);
    storage.cleanup().await?;
    Ok(storage)
}
