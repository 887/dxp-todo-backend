use axum::{extract::Json, http::StatusCode, routing::put, Router};
use serde::{Deserialize, Serialize};
use tracing::trace;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Test {
    pub test: String,
}

#[utoipa::path(
    put,
    path = "/api/test",
    tag = "Test",
    responses(
        (status = 200, description = "Test operation successful", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    params(
        ("test", description = "Json<Test>")
    )
)]
pub async fn test_put(Json(test): Json<Test>) -> Result<String, (StatusCode, String)> {
    trace!("/test_put");
    let t = test.test;
    Ok(format!("test:{}", t))
}

pub fn routes() -> Router {
    Router::new().route("/test", put(test_put))
}
