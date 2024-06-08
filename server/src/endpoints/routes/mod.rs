use anyhow::Result;

use poem::{get, IntoEndpoint, Route};

mod index;

pub(crate) async fn get_route() -> Result<impl IntoEndpoint> {
    Ok(Route::new().at("/", get(index::index)))
}
