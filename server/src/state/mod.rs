use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::session::SessionStorageType;

#[derive(Clone, Debug)]
pub struct State {
    pub db: DatabaseConnection,
    pub storage: SessionStorageType,
}

impl State {
    pub async fn new(db: DatabaseConnection, storage: SessionStorageType) -> Result<State> {
        Ok(State { db, storage })
    }
}
