use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use axum_session::DatabasePool;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;
use utoipa::ToSchema;

use crate::state::State;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthenticateApi;

#[derive(Serialize, ToSchema)]
pub enum AuthenticationResult {
    #[schema(example = "session_id")]
    Some(String),
    Conflict,
    Forbidden,
}

#[utoipa::path(
    put,
    path = "/login",
    tag = "Authenticate",
    operation_id = "authenticate",
    params(
        ("user_name" = String, Query, description = "User name"),
        ("password" = String, Query, description = "Password"),
        ("device_info" = String, Query, description = "Device info")
    ),
    responses(
        (status = 200, description = "Authentication successful", body = String),
        (status = 409, description = "Conflict", body = String),
        (status = 401, description = "Forbidden", body = String)
    ),
    security(
        ("ApiKeyAuth" = [])
    )
)]
async fn login(
    Extension(state): Extension<State>,
    Query(user_name): Query<String>,
    Query(password): Query<String>,
    Query(_device_info): Query<String>,
) -> Result<Json<AuthenticationResult>, (StatusCode, String)> {
    if password != "password" {
        return Ok(Json(AuthenticationResult::Forbidden));
    }
    let session_pool = &state.session_pool;

    let session_id = generate_session_id();
    let session = session_pool
        .load(&session_id, dxp_axum_session::TABLE_NAME)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if session.is_some() {
        return Ok(Json(AuthenticationResult::Conflict));
    }

    //TODO: user_id UUID-v7 from db, check password, implement register etc,
    //TODO: insert session id + device info + user id in db

    let mut map = serde_json::Map::new();

    map.insert("user_name".to_string(), Value::String(user_name));
    let value = serde_json::Value::Object(map);

    let session: &str = &value.to_string();
    session_pool
        .store(
            &session_id,
            session,
            60 * 60 * 24 * 365,
            dxp_axum_session::TABLE_NAME,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    trace!("login");

    Ok(Json(AuthenticationResult::Some(session_id)))
}

fn generate_session_id() -> String {
    let random_bytes = thread_rng().gen::<[u8; 32]>();
    BASE64_URL_SAFE_NO_PAD.encode(random_bytes)
}
