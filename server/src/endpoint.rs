use crate::routes;
use crate::session;

use anyhow::Result;
use axum::Extension;
use axum::Router;
use sea_orm::DatabaseConnection;
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

    Ok(router.layer(CompressionLayer::new()).layer(router)).layer(Extension(db))
}
