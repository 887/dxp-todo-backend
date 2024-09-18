use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::session::SessionPoolType;

#[derive(Clone, Debug)]
pub struct State {
    pub db: DatabaseConnection,
    pub session_pool: SessionPoolType,
}

impl State {
    pub async fn new(db: DatabaseConnection, session_pool: SessionPoolType) -> Result<State> {
        Ok(State { db, session_pool })
    }
}
