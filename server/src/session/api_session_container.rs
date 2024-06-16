use super::{
    api_session::{ApiSession, ApiSessionStatus},
    SessionStorageType,
};
use poem::session::SessionStorage;

use tracing::error;

#[derive(Clone, Debug)]
pub struct ApiSessionContainer {
    /// The api key serves as the session_id
    key: String,
    pub session: ApiSession,
    storage: SessionStorageType,
    updated: bool,
}

impl Drop for ApiSessionContainer {
    fn drop(&mut self) {
        if self.updated || self.session.status() == ApiSessionStatus::Unchanged {
            return;
        }
        error!("key: {} - changes not updated", self.key);
    }
}

impl ApiSessionContainer {
    pub fn new(key: String, session: ApiSession, storage: SessionStorageType) -> Self {
        Self {
            key,
            session,
            storage,
            updated: false,
        }
    }

    pub async fn update(&mut self) -> Result<(), poem::Error> {
        let session_id = &self.key;
        let session = &self.session;
        let res = match self.session.status() {
            ApiSessionStatus::Changed => {
                self.storage
                    .update_session(session_id, &session.entries(), None)
                    .await
            }
            ApiSessionStatus::Purged => self.storage.remove_session(session_id).await,
            ApiSessionStatus::Unchanged => Ok(()),
        };

        self.updated = true;
        res
    }
}
