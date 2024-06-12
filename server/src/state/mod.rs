use anyhow::Result;
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct State {
    pub db: DatabaseConnection,
}

impl State {
    pub async fn new(db: DatabaseConnection) -> Result<State> {
        Ok(State { db })
    }
}
