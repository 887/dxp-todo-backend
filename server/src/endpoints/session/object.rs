use poem::session::SessionStorage;

#[derive(Clone)]
pub struct SessionStorageObject {
    #[cfg(not(any(
        feature = "mysql-rustls",
        feature = "mysql-native-tls",
        feature = "sqlite-rustls",
        feature = "sqlite-native-tls",
        feature = "postgres-rustls",
        feature = "postgres-native-tls",
        feature = "redis"
    )))]
    pub storage: poem::session::MemoryStorage,
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
    pub storage: dxp_db_session::DbSessionStorage,
    #[cfg(feature = "redis")]
    pub storage: poem::session::RedisStorage,
}

impl SessionStorage for SessionStorageObject {
    async fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> poem::Result<Option<std::collections::BTreeMap<String, serde_json::Value>>> {
        self.storage.load_session(&session_id).await
    }

    async fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        entries: &'a std::collections::BTreeMap<String, serde_json::Value>,
        expires: Option<std::time::Duration>,
    ) -> poem::Result<()> {
        self.storage
            .update_session(session_id, entries, expires)
            .await
    }

    async fn remove_session<'a>(&'a self, session_id: &'a str) -> poem::Result<()> {
        self.storage.remove_session(session_id).await
    }
}
