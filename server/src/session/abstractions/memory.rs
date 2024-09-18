pub type SessionPoolType = axum_session::SessionNullPool;

pub async fn get_pool() -> anyhow::Result<SessionNullPool> {
    Ok(axum_session::SessionNullPool {})
}
