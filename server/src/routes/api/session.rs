///WARNING: If you expose the API, secure these endpoints so only the frontend can access them. These are for internal use only.
///In a client side application you won't need these at all.
//TODO pre-secure these with a shared header key in .env? Compile time feature?
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
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
    pub expires: Option<u64>,
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

#[utoipa::path(
    get,
    path = "/load_session",
    tag = "Session",
    operation_id = "load_session",
    params(
        ("session_id" = String, Query, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session found", body = OptionalResponse<BTreeMap<String, Value>>),
        (status = 404, description = "Session not found", body = OptionalResponse<BTreeMap<String, Value>>)
    )
)]
async fn load_session(
    Extension(session): Extension<DatabasePoolObject>,
    Query(session_id): Query<String>,
) -> Result<Json<OptionalResponse<Map<String, Value>>>, (StatusCode, String)> {
    trace!("/load_session");
    let session_id = frontend_session_id(session_id);
    let entries = session
        .load(&session_id, TABLE_NAME)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let entries = entries
        .map(|entries| {
            serde_json::from_str(&entries)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        })
        .transpose()?;
    match entries {
        Some(entries) => Ok(Json(OptionalResponse::Some(entries))),
        None => Ok(Json(OptionalResponse::None)),
    }
}

#[utoipa::path(
    put,
    path = "/update_session",
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
    Query(session_id): Query<String>,
    Json(value): Json<UpdateSessionValue>,
) -> Result<(), (StatusCode, String)> {
    trace!("/update_session");
    let entries = value.entries;
    let expires = value.expires;

    let session_id = frontend_session_id(session_id);

    let session_value = serde_json::to_string(&entries)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    session
        .store(
            &session_id,
            &session_value,
            expires
                .map(|e| e as i64)
                .unwrap_or(Utc::now().timestamp() + 60 * 60 * 24 * 365),
            TABLE_NAME,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[utoipa::path(
    delete,
    path = "/remove_session",
    tag = "Session",
    operation_id = "remove_session",
    params(
        ("session_id" = String, Query, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session removed"),
        (status = 500, description = "Internal server error")
    )
)]
async fn remove_session(
    Extension(session): Extension<DatabasePoolObject>,
    Query(session_id): Query<String>,
) -> Result<(), (StatusCode, String)> {
    let session_id = frontend_session_id(session_id);

    trace!("/remove_session");
    session
        .remove_session(&session_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/load_session", get(load_session))
        .route("/update_session", put(update_session))
        .route("/remove_session", delete(remove_session))
}
