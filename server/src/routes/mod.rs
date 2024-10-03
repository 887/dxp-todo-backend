use anyhow::Result;

use axum::{routing::get, Router};

pub mod api;
pub mod hot;
mod index;
pub mod swagger_ui;

pub(crate) async fn get_route() -> Result<Router> {
    Ok(Router::new()
        .route("/", get(index::index))
        .route("/2", get(index::index2)))
}
