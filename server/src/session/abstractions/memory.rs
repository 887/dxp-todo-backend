pub type SessionPoolType = axum_session::SessionNullPool;

pub type SessionType = axum_session::Session<SessionPoolType>;

pub async fn get_pool() -> anyhow::Result<SessionNullPool> {
    Ok(axum_session::SessionNullPool {})
}
