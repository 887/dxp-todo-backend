use std::io::Write;

use crate::routes;
use crate::session;
use anyhow::Result;
use axum::body::to_bytes;
use axum::body::Body;
use axum::http::header;
use axum::http::header::HeaderMap;
use axum::http::header::HeaderValue;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use axum::{middleware::from_fn, Extension};
use sea_orm::DatabaseConnection;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use tower_http::compression::CompressionLayer;

pub async fn get_route(db: DatabaseConnection) -> Result<Router> {
    //https://github.com/AscendingCreations/AxumSession
    let session_storage = session::storage::get_storage(db.clone()).await?;
    let session_middleware = session::get_session_middleware(session_storage)?;

    let mut router = Router::new()
        .nest("/", routes::get_route().await?)
        .nest("/hot", routes::hot::get_route());

    //go to http://127.0.0.1:8000/swagger
    #[cfg(feature = "swagger-ui")]
    {
        router = router.nest(
            "/swagger",
            routes::swagger_ui::get_route(Some("/api/swagger.json")),
        );
    }

    let api_service = routes::api::get_api_service("http://127.0.0.1:8000");
    router = router.nest(
        "/api",
        routes::api::get_route(api_service, db.clone()).await?,
    );

    Ok(router.layer(CompressionLayer::new()).layer(layer)).layer(Extension(db))
}
