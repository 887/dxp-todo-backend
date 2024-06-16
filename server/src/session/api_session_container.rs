use super::{
    api_session::{ApiSession, ApiSessionStatus},
    SessionStorageType,
};
use poem::session::SessionStorage;

#[derive(Clone, Debug)]
pub struct ApiSessionContainer {
    pub(crate) key: String,
    pub session: ApiSession,
    pub(crate) storage: SessionStorageType,
}

impl ApiSessionContainer {
    pub async fn update(&self) -> Result<(), poem::Error> {
        let session_id = &self.key;
        let session = &self.session;
        match self.session.status() {
            ApiSessionStatus::Changed => {
                self.storage
                    .update_session(session_id, &session.entries(), None)
                    .await
            }
            ApiSessionStatus::Purged => self.storage.remove_session(session_id).await,
            ApiSessionStatus::Unchanged => Ok(()),
        }
    }
}
