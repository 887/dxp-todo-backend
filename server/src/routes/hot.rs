use axum::routing::get;
use axum::{extract::Extension, response::IntoResponse, Router};
use chrono::Utc;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct HotVersion {
    pub data: i64,
}

async fn loaded_version(Extension(hot_version): Extension<Arc<HotVersion>>) -> impl IntoResponse {
    hot_version.data.to_string()
}

pub fn get_route() -> Router {
    let rn = Utc::now().timestamp();
    let hot_version = Arc::new(HotVersion { data: rn });

    Router::new()
        .route("/", get(loaded_version))
        .layer(Extension(hot_version))
}
