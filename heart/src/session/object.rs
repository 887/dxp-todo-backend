//object to wrap the trait to make it accessible to the endpoints as data

use axum::async_trait;
use axum_session::DatabasePool;

use super::SessionPoolType;

#[derive(Clone)]
pub struct DatabasePoolObject {
    pub storage: SessionPoolType,
}

#[async_trait]
impl DatabasePool for DatabasePoolObject {
    #[inline(always)]
    async fn initiate(&self, table_name: &str) -> Result<(), axum_session::DatabaseError> {
        self.storage.initiate(table_name).await
    }

    #[inline(always)]
    async fn delete_by_expiry(
        &self,
        table_name: &str,
    ) -> Result<Vec<String>, axum_session::DatabaseError> {
        self.storage.delete_by_expiry(table_name).await
    }

    #[inline(always)]
    async fn count(&self, table_name: &str) -> Result<i64, axum_session::DatabaseError> {
        self.storage.count(table_name).await
    }

    #[inline(always)]
    async fn store(
        &self,
        id: &str,
        session: &str,
        expires: i64,
        table_name: &str,
    ) -> Result<(), axum_session::DatabaseError> {
        self.storage.store(id, session, expires, table_name).await
    }

    #[inline(always)]
    async fn load(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<Option<String>, axum_session::DatabaseError> {
        self.storage.load(id, table_name).await
    }

    #[inline(always)]
    async fn delete_one_by_id(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<(), axum_session::DatabaseError> {
        self.storage.delete_one_by_id(id, table_name).await
    }

    #[inline(always)]
    async fn exists(
        &self,
        id: &str,
        table_name: &str,
    ) -> Result<bool, axum_session::DatabaseError> {
        self.storage.exists(id, table_name).await
    }

    #[inline(always)]
    async fn delete_all(&self, table_name: &str) -> Result<(), axum_session::DatabaseError> {
        self.storage.delete_all(table_name).await
    }

    #[inline(always)]
    async fn get_ids(&self, table_name: &str) -> Result<Vec<String>, axum_session::DatabaseError> {
        self.storage.get_ids(table_name).await
    }

    #[inline(always)]
    fn auto_handles_expiry(&self) -> bool {
        self.storage.auto_handles_expiry()
    }
}

// impl SessionStorage for SessionStorageObject {
//     async fn load_session<'a>(
//         &'a self,
//         session_id: &'a str,
//     ) -> poem::Result<Option<std::collections::BTreeMap<String, serde_json::Value>>> {
//         self.storage.load_session(session_id).await
//     }

//     async fn update_session<'a>(
//         &'a self,
//         session_id: &'a str,
//         entries: &'a std::collections::BTreeMap<String, serde_json::Value>,
//         expires: Option<std::time::Duration>,
//     ) -> poem::Result<()> {
//         self.storage
//             .update_session(session_id, entries, expires)
//             .await
//     }

//     async fn remove_session<'a>(&'a self, session_id: &'a str) -> poem::Result<()> {
//         self.storage.remove_session(session_id).await
//     }
// }
