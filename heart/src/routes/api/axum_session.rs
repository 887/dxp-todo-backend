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
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// maybe useful for future reference
// https://github.com/DioxusLabs/dioxus/blob/main/examples/fullstack-auth/src/main.rs

use crate::session::DatabasePoolObject;

#[derive(Deserialize, Serialize)]
pub struct DeleteByExpiryParams {
    pub table_name: String,
}

#[utoipa::path(
    delete,
    path = "/api/session/delete_by_expiry",
    tag = "Session",
    operation_id = "delete_by_expiry",
    params(
        ("table_name" = String, Query, description = "table name")
    ),
    responses(
        (status = 200, description = "worked", body = Vec<String>),
    )
)]
async fn delete_by_expiry(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<DeleteByExpiryParams>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let DeleteByExpiryParams { table_name } = params;
    pool.delete_by_expiry(&table_name)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct CountParams {
    pub table_name: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CountResponse {
    pub count: i64,
}

#[utoipa::path(
    get,
    path = "/api/session/count",
    tag = "Session",
    operation_id = "count",
    params(
        ("table_name" = String, Query, description = "table name")
    ),
    responses(
        (status = 200, description = "worked", body = CountResponse, example = json!(CountResponse { count: 0 })),
    )
)]
#[axum::debug_handler]
async fn count(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<CountParams>,
) -> Result<Json<CountResponse>, (StatusCode, String)> {
    let CountParams { table_name } = params;
    pool.count(&table_name)
        .await
        .map(|count| CountResponse { count })
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct StoreParams {
    pub id: String,
    pub session: String,
    pub expires: i64,
    pub table_name: String,
}

#[utoipa::path(
    put,
    path = "/api/session/store",
    tag = "Session",
    operation_id = "store",
    params(
        ("id" = String, Query, description = "Session ID"),
        ("session" = String, Query, description = "Session data"),
        ("expires" = i64, Query, description = "Expiration timestamp"),
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked"),
    )
)]
async fn store(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<StoreParams>,
) -> Result<(), (StatusCode, String)> {
    let StoreParams {
        id,
        session,
        expires,
        table_name,
    } = params;
    pool.store(&id, &session, expires, &table_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct LoadParams {
    pub id: String,
    pub table_name: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoadResponse {
    pub value: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/session/load",
    tag = "Session",
    operation_id = "load",
    params(
        ("id" = String, Query, description = "Session ID"),
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked", body = LoadResponse, example = json!(LoadResponse { value: Some("".to_string()) })),
        (status = 404, description = "worked"),

    )
)]
async fn load(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<LoadParams>,
) -> Result<Json<LoadResponse>, (StatusCode, String)> {
    let LoadParams { id, table_name } = params;
    match pool.load(&id, &table_name).await {
        Ok(value) => Ok(Json(LoadResponse { value })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeleteOneByIdParams {
    pub id: String,
    pub table_name: String,
}

#[utoipa::path(
    delete,
    path = "/api/session/delete_one_by_id",
    tag = "Session",
    operation_id = "delete_one_by_id",
    params(
        ("id" = String, Query, description = "Session ID"),
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked"),

    )
)]
async fn delete_one_by_id(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<DeleteOneByIdParams>,
) -> Result<(), (StatusCode, String)> {
    let DeleteOneByIdParams { id, table_name } = params;
    pool.delete_one_by_id(&id, &table_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct ExistsParams {
    pub id: String,
    pub table_name: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ExistsResponse {
    pub value: bool,
}

#[utoipa::path(
    get,
    path = "/api/session/exists",
    tag = "Session",
    operation_id = "exists",
    params(
        ("id" = String, Query, description = "Session ID"),
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked", body = ExistsResponse, example = json!(ExistsResponse { value: false })),

    )
)]
async fn exists(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<ExistsParams>,
) -> Result<Json<ExistsResponse>, (StatusCode, String)> {
    let ExistsParams { id, table_name } = params;
    pool.exists(&id, &table_name)
        .await
        .map(|value| ExistsResponse { value })
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct DeleteAllParams {
    pub table_name: String,
}

#[utoipa::path(
    delete,
    path = "/api/session/delete_all",
    tag = "Session",
    operation_id = "delete_all",
    params(
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked"),

    )
)]
async fn delete_all(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<DeleteAllParams>,
) -> Result<(), (StatusCode, String)> {
    let DeleteAllParams { table_name } = params;
    pool.delete_all(&table_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct GetIdsParams {
    pub table_name: String,
}

#[utoipa::path(
    get,
    path = "/api/session/get_ids",
    tag = "Session",
    operation_id = "get_ids",
    params(
        ("table_name" = String, Query, description = "Table name")
    ),
    responses(
        (status = 200, description = "worked", body = Vec<String>),
    )
)]
async fn get_ids(
    Extension(pool): Extension<DatabasePoolObject>,
    Query(params): Query<GetIdsParams>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let GetIdsParams { table_name } = params;
    pool.get_ids(&table_name)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub fn routes() -> Router {
    Router::new()
        .route("/session/delete_by_expiry", delete(delete_by_expiry))
        .route("/session/count", get(count))
        .route("/session/store", put(store))
        .route("/session/load", get(load))
        .route("/session/delete_one_by_id", delete(delete_one_by_id))
        .route("/session/exists", get(exists))
        .route("/session/delete_all", delete(delete_all))
        .route("/session/get_ids", get(get_ids))
}
