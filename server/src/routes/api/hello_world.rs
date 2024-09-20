use axum::{extract::Query, http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize};
use tracing::trace;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct HelloWorldApi;

#[derive(ToSchema)]
enum Tags {
    /// HelloWorld operations
    HelloWorld,
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = "Tags::HelloWorld",
    operation_id = "hello",
    responses(
        (status = 200, description = "Say hello", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn hello() -> Result<String, (StatusCode, String)> {
    trace!("/hello");
    Ok("Hello, World!".to_string())
}

#[utoipa::path(
    get,
    path = "/greet",
    tag = "Tags::HelloWorld",
    operation_id = "greet",
    params(
        ("name" = Option<String>, Query, description = "Name to greet")
    ),
    responses(
        (status = 200, description = "Greetings", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn greet(Query(name): Query<Option<String>>) -> Result<String, (StatusCode, String)> {
    trace!("/greet");
    let greeting = match name {
        Some(name) => format!("hello, {}!", name),
        None => "hello!".to_string(),
    };
    Ok(greeting)
}

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/greet", get(greet))
}
