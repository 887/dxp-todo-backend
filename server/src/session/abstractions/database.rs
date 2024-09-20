pub type SessionPoolType = dxp_axum_session::DbPool;

pub type SessionType = axum_session::Session<SessionPoolType>;

pub async fn get_pool(db: sea_orm::DatabaseConnection) -> anyhow::Result<SessionPoolType> {
    Ok(dxp_axum_session::DbPool::new(db))
}
