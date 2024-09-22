use anyhow::Context;
use axum::{extract::Json, http::StatusCode, routing::post, Router};
use serde::{Deserialize, Serialize};
use tracing::trace;
use utoipa::ToSchema;

use crate::{error::LogErrExt, session::SessionType, state::State};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Todo {
    pub test: String,
}

#[derive(ToSchema)]
pub enum Tags {
    /// HelloWorld operations
    Todo,
}

//https://github.com/codemountains/utoipa-example-with-axum/blob/main/src/main.rs

#[utoipa::path(
    put,
    path = "/todo",
    tag = "Todo",
    responses(
        (status = 200, description = "Todo item created successfully", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    params(
        ("todo", description = "Json<Todo>")
    )
)]
pub async fn todo_put(
    session: SessionType,
    Json(todo): Json<Todo>,
    state: axum::extract::Extension<State>,
) -> Result<String, (StatusCode, String)> {
    trace!("/todo_put");

    //todo implement todo api
    state
        .db
        .ping()
        .await
        .context("Failed to ping database")
        .log_error()
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)))?;

    session.set("name", "name");

    session.update();

    let t = todo.test;
    Ok(format!("todo_put:{}", t))
}

pub fn routes() -> Router {
    Router::new().route("/todo", post(todo_put))
}
