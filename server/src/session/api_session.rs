use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tracing::error;

use super::SessionPoolType;

use axum_session::DatabasePool;

const TABLE_NAME: &str = "sessions";

#[derive(Debug)]
pub struct ApiSession {
    /// The api key serves as the session_id
    key: String,
    status: ApiSessionStatus,
    pub session: String,
    pool: SessionPoolType,
}

// this is a development feature to ensure that all sessions changes are saved before they are dropped
#[cfg(debug_assertions)]
impl Drop for ApiSession {
    fn drop(&mut self) {
        if self.status == ApiSessionStatus::Unchanged {
            return;
        }
        error!("key: {} - changes not updated", self.key);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ApiSessionStatus {
    /// Indicates that the session state has changed.
    Changed,
    /// Indicates that the session state needs to be cleared.
    Purged,
    /// Indicates that the session state is unchanged.
    Unchanged,
}

#[allow(dead_code)]
impl ApiSession {
    /// Creates a new session instance.
    ///
    /// The default status is [`SessionStatus::Unchanged`].
    pub(crate) fn new(key: String, pool: SessionPoolType, session: String) -> Self {
        Self {
            key,
            pool,
            status: ApiSessionStatus::Unchanged,
            session,
        }
    }

    pub async fn update(&mut self) -> anyhow::Result<()> {
        let session_id = &self.key;
        match self.status {
            ApiSessionStatus::Changed => {
                let res = self
                    .pool
                    .store(session_id, &self.session, 3600, "sessions")
                    .await?;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }
                Ok(res)
            }
            ApiSessionStatus::Purged => {
                let res = self.pool.delete_one_by_id(session_id, "").await?;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }
                Ok(res)
            }
            ApiSessionStatus::Unchanged => Ok(()),
        }
    }

    /// Get a value from the session.
    pub fn get<T: DeserializeOwned>(&self, name: &str) -> Option<T> {
        todo!();
        self.session
            .get(name)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Sets a key-value pair into the session.
    pub fn set(&mut self, name: &str, value: impl Serialize) {
        if self.status != ApiSessionStatus::Purged {
            if let Ok(value) = serde_json::to_value(&value) {
                todo!();
                self.session.insert(name.to_string(), value);
                self.status = ApiSessionStatus::Changed;
            }
        }
    }

    /// Remove value from the session.
    pub fn remove(&mut self, name: &str) {
        if self.status != ApiSessionStatus::Purged {
            todo!();
            self.session.remove(name);
            self.status = ApiSessionStatus::Changed;
        }
    }

    /// Returns `true` is this session does not contain any values, otherwise it
    /// returns `false`.
    pub fn is_empty(&self) -> bool {
        todo!();
        self.session.is_empty()
    }

    /// Get all raw key-value data from the session
    pub fn entries(&self) -> String {
        self.session.clone()
    }

    /// Clear the session.
    pub async fn clear(&mut self) {
        if self.status != ApiSessionStatus::Purged {
            self.session.clear();
            self.status = ApiSessionStatus::Changed;
        }
    }

    /// Removes session both client and server side.
    pub fn purge(&mut self) {
        if self.status != ApiSessionStatus::Purged {
            self.session.clear();
            self.status = ApiSessionStatus::Purged;
        }
    }

    /// Returns the status of this session.
    pub fn status(&mut self) -> ApiSessionStatus {
        self.status
    }
}
