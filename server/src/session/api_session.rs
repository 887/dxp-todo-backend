use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
#[allow(unused_imports)]
use tracing::error;

use super::SessionPoolType;

use axum_session::DatabasePool;

#[derive(Debug)]
pub struct ApiSession {
    /// The api key serves as the session_id
    key: String,
    status: ApiSessionStatus,
    pub session: String,
    pool: SessionPoolType,
}

impl ApiSession {
    pub fn get_session_as_json(&self) -> Result<serde_json::Map<String, Value>, serde_json::Error> {
        if self.session.is_empty() {
            Ok(serde_json::Map::new())
        } else {
            serde_json::from_str(&self.session)
        }
    }
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
                self.pool
                    .store(session_id, &self.session, 3600, "sessions")
                    .await?;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }
                Ok(())
            }
            ApiSessionStatus::Purged => {
                self.pool.delete_one_by_id(session_id, "").await?;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }
                Ok(())
            }
            ApiSessionStatus::Unchanged => Ok(()),
        }
    }

    /// Get a value from the session.
    pub fn get<T: DeserializeOwned>(&self, name: &str) -> Option<T> {
        let json = self.get_session_as_json().ok()?;
        json.get(name)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Sets a key-value pair into the session.
    pub fn set(&mut self, name: &str, value: impl Serialize) {
        if self.status != ApiSessionStatus::Purged {
            if let Ok(value) = serde_json::to_value(&value) {
                if let Ok(mut json) = self.get_session_as_json() {
                    json.insert(name.to_string(), value);
                    self.session = serde_json::to_string(&json).unwrap_or_default();
                    self.session = serde_json::to_string(&json).unwrap_or_default();
                    self.status = ApiSessionStatus::Changed;
                }
            }
        }
    }

    /// Remove value from the session.
    pub fn remove(&mut self, name: &str) {
        if self.status != ApiSessionStatus::Purged {
            if let Ok(mut json) = self.get_session_as_json() {
                json.remove(name);
                self.session = serde_json::to_string(&json).unwrap_or_default();
                self.session = serde_json::to_string(&json).unwrap_or_default();
                self.status = ApiSessionStatus::Changed;
            }
        }
    }

    /// Returns `true` is this session does not contain any values, otherwise it
    /// returns `false`.
    pub fn is_empty(&self) -> bool {
        if self.session.is_empty() {
            true
        } else if let Ok(json) = self.get_session_as_json() {
            json.is_empty()
        } else {
            false
        }
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
