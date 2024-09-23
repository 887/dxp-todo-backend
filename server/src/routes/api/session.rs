///WARNING: If you expose the API, secure these endpoints so only the frontend can access them. These are for internal use only.
///In a client side application you won't need these at all.
//TODO pre-secure these with a shared header key in .env? Compile time feature?
use axum::{
    extract::{Extension, Query},
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::{delete, get, put},
    Json, Router,
};
use axum_session::DatabasePool;
use chrono::Utc;
use dxp_axum_session::TABLE_NAME;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use tracing::trace;
use utoipa::ToSchema;

use crate::session::DatabasePoolObject;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateSessionValue {
    pub entries: Map<String, Value>,
    #[serde(default = "get_default_expires_value")]
    pub expires: u64,
}

#[derive(Serialize, ToSchema)]
pub enum OptionalResponse<T> {
    #[schema(example = "Some")]
    Some(T),
    #[schema(example = "None")]
    None,
}

fn frontend_session_id(session_id: String) -> String {
    ["fe_", &session_id].concat()
}

#[derive(Deserialize, Debug)]
pub struct LoadSessionParams {
    session_id: String,
}

#[utoipa::path(
    get,
    path = "/api/load_session",
    tag = "Session",
    operation_id = "load_session",
    params(
        ("session_id" = String, Query, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session found", body = Option<String>),
        (status = 404, description = "Session not found", body = Option<String>)
    )
)]
async fn load_session(
    Extension(session): Extension<DatabasePoolObject>,
    Query(params): Query<LoadSessionParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    trace!("/load_session");
    let session_id = frontend_session_id(params.session_id);
    let entries = session
        .load(&session_id, TABLE_NAME)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match entries {
        Some(entries) => Ok(([(CONTENT_TYPE, "application/json")], entries)),
        None => Err((StatusCode::NOT_FOUND, "Session not found".to_string())),
    }
}

fn get_default_expires_value() -> u64 {
    (Utc::now() + chrono::Duration::days(365)).timestamp() as u64
}

#[derive(Deserialize, Debug)]
pub struct UpdateSessionParams {
    session_id: String,
}

#[utoipa::path(
    put,
    path = "/api/update_session",
    tag = "Session",
    operation_id = "update_session",
    params(
        ("session_id" = String, Query, description = "Session ID")
    ),
    request_body = UpdateSessionValue,
    responses(
        (status = 200, description = "Session updated"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_session(
    Extension(session): Extension<DatabasePoolObject>,
    Query(params): Query<UpdateSessionParams>,
    Json(value): Json<UpdateSessionValue>,
) -> Result<(), (StatusCode, String)> {
    trace!("/api/update_session");
    let entries = value.entries;
    let expires = value.expires;

    let session_id = frontend_session_id(params.session_id);

    let session_value = serde_json::to_string(&entries)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let expires = match expires {
        0 => get_default_expires_value(),
        value => value,
    } as i64;

    session
        .store(&session_id, &session_value, expires, TABLE_NAME)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct RemoveSessionParams {
    session_id: String,
}

#[utoipa::path(
    delete,
    path = "/api/remove_session",
    tag = "Session",
    operation_id = "remove_session",
    params(
        ("session_id" = RemoveSessionParams, Query, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session removed"),
        (status = 500, description = "Internal server error")
    )
)]
async fn remove_session(
    Extension(session): Extension<DatabasePoolObject>,
    Query(params): Query<RemoveSessionParams>,
) -> Result<(), (StatusCode, String)> {
    let session_id = frontend_session_id(params.session_id);

    trace!("/remove_session");
    session
        .delete_one_by_id(&session_id, TABLE_NAME)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/load_session", get(load_session))
        .route("/update_session", put(update_session))
        .route("/remove_session", delete(remove_session))
}
