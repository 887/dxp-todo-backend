use anyhow::Result;

use poem::{get, IntoEndpoint, Route};

pub mod api;
pub mod hot;
mod index;
pub mod swagger_ui;

pub(crate) async fn get_route() -> Result<impl IntoEndpoint> {
    Ok(Route::new().at("/", get(index::index)))
}
