use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tracing::error;

use super::SessionStorageType;

use poem::session::SessionStorage;

#[derive(Debug)]
pub struct ApiSession {
    /// The api key serves as the session_id
    key: String,
    status: ApiSessionStatus,
    pub entries: BTreeMap<String, Value>,
    storage: SessionStorageType,
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
    pub(crate) fn new(
        key: String,
        storage: SessionStorageType,
        entries: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            key,
            storage,
            status: ApiSessionStatus::Unchanged,
            entries,
        }
    }

    pub async fn update(&mut self) -> Result<(), poem::Error> {
        let session_id = &self.key;
        match self.status {
            ApiSessionStatus::Changed => {
                let res = self
                    .storage
                    .update_session(session_id, &self.entries, None)
                    .await;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }
                res
            }
            ApiSessionStatus::Purged => {
                let res = self.storage.remove_session(session_id).await;

                #[cfg(debug_assertions)]
                {
                    self.status = ApiSessionStatus::Unchanged;
                }

                res
            }
            ApiSessionStatus::Unchanged => Ok(()),
        }
    }

    /// Get a value from the session.
    pub fn get<T: DeserializeOwned>(&self, name: &str) -> Option<T> {
        self.entries
            .get(name)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Sets a key-value pair into the session.
    pub fn set(&mut self, name: &str, value: impl Serialize) {
        if self.status != ApiSessionStatus::Purged {
            if let Ok(value) = serde_json::to_value(&value) {
                self.entries.insert(name.to_string(), value);
                self.status = ApiSessionStatus::Changed;
            }
        }
    }

    /// Remove value from the session.
    pub fn remove(&mut self, name: &str) {
        if self.status != ApiSessionStatus::Purged {
            self.entries.remove(name);
            self.status = ApiSessionStatus::Changed;
        }
    }

    /// Returns `true` is this session does not contain any values, otherwise it
    /// returns `false`.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all raw key-value data from the session
    pub fn entries(&self) -> BTreeMap<String, Value> {
        self.entries.clone()
    }

    /// Clear the session.
    pub async fn clear(&mut self) {
        if self.status != ApiSessionStatus::Purged {
            self.entries.clear();
            self.status = ApiSessionStatus::Changed;
        }
    }

    /// Removes session both client and server side.
    pub fn purge(&mut self) {
        if self.status != ApiSessionStatus::Purged {
            self.entries.clear();
            self.status = ApiSessionStatus::Purged;
        }
    }

    /// Returns the status of this session.
    pub fn status(&mut self) -> ApiSessionStatus {
        self.status
    }
}
