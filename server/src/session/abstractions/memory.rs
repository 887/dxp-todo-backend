pub type SessionPoolType = dxp_axum_session::MemoryPool;

pub type SessionType = axum_session::Session<SessionPoolType>;

pub async fn get_pool() -> anyhow::Result<SessionNullPool> {
    Ok(SessionPoolType::new())
}
