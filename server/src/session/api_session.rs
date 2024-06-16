use std::{collections::BTreeMap, sync::Arc};

use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[derive(Debug)]
struct ApiSessionInner {
    pub status: ApiSessionStatus,
    pub entries: BTreeMap<String, Value>,
}

#[derive(Debug)]
pub struct ApiSession {
    inner: Arc<RwLock<ApiSessionInner>>,
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

impl Default for ApiSession {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[allow(dead_code)]
impl ApiSession {
    /// Creates a new session instance.
    ///
    /// The default status is [`SessionStatus::Unchanged`].
    pub(crate) fn new(entries: BTreeMap<String, Value>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ApiSessionInner {
                status: ApiSessionStatus::Unchanged,
                entries,
            })),
        }
    }

    /// Get a value from the session.
    pub fn get<T: DeserializeOwned>(&self, name: &str) -> Option<T> {
        let inner = self.inner.read();
        inner
            .entries
            .get(name)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Sets a key-value pair into the session.
    pub fn set(&self, name: &str, value: impl Serialize) {
        let mut inner = self.inner.write();
        if inner.status != ApiSessionStatus::Purged {
            if let Ok(value) = serde_json::to_value(&value) {
                inner.entries.insert(name.to_string(), value);
                inner.status = ApiSessionStatus::Changed;
            }
        }
    }

    /// Remove value from the session.
    pub fn remove(&self, name: &str) {
        let mut inner = self.inner.write();
        if inner.status != ApiSessionStatus::Purged {
            inner.entries.remove(name);
            inner.status = ApiSessionStatus::Changed;
        }
    }

    /// Returns `true` is this session does not contain any values, otherwise it
    /// returns `false`.
    pub fn is_empty(&self) -> bool {
        let inner = self.inner.read();
        inner.entries.is_empty()
    }

    /// Get all raw key-value data from the session
    pub fn entries(&self) -> BTreeMap<String, Value> {
        let inner = self.inner.read();
        inner.entries.clone()
    }

    /// Clear the session.
    pub fn clear(&self) {
        let mut inner = self.inner.write();
        if inner.status != ApiSessionStatus::Purged {
            inner.entries.clear();
            inner.status = ApiSessionStatus::Changed;
        }
    }

    /// Removes session both client and server side.
    pub fn purge(&self) {
        let mut inner = self.inner.write();
        if inner.status != ApiSessionStatus::Purged {
            inner.entries.clear();
            inner.status = ApiSessionStatus::Purged;
        }
    }

    /// Returns the status of this session.
    pub fn status(&self) -> ApiSessionStatus {
        let inner = self.inner.read();
        inner.status
    }
}
