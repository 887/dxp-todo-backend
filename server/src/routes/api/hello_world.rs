use axum::{extract::Query, http::StatusCode, routing::get, Router};
use serde::Deserialize;
use tracing::trace;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/hello",
    tag = "HelloWorld",
    operation_id = "hello",
    responses(
        (status = 200, description = "Say hello", body = String),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn hello() -> Result<String, (StatusCode, String)> {
    trace!("/hello");
    Ok("Hello, World!".to_string())
}

fn default_none<T>() -> Option<T> {
    None
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct GreetParams {
    #[serde(default = "default_none")]
    name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/greet",
    tag = "HelloWorld",
    operation_id = "greet",
    params(
        ("name" = Option<String>, Query, description = "Name to greet")
    ),
    responses(
        (status = 200, description = "Greetings", body = String),
        (status = 500, description = "Internal server error",)
    )
)]
pub async fn greet(Query(params): Query<GreetParams>) -> Result<String, (StatusCode, String)> {
    trace!("/greet");
    let greeting = match params.name {
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
