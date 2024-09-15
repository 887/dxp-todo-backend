use poem::session::SessionStorage;

use super::SessionStorageType;

//TODO might be solvable with this now:
use axum_session::SessionAnyPool;

#[derive(Clone)]
pub struct SessionStorageObject {
    pub storage: SessionStorageType,
}

impl SessionStorage for SessionStorageObject {
    async fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> poem::Result<Option<std::collections::BTreeMap<String, serde_json::Value>>> {
        self.storage.load_session(session_id).await
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
