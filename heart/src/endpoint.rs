use crate::routes;
use crate::session;
use crate::session::set_cookie_key;

use anyhow::Result;
use axum::Extension;
use axum::Router;
use axum_session::SessionConfig;
use axum_session::SessionLayer;
use axum_session::SessionStore;
use sea_orm::DatabaseConnection;
use tower_http::compression::CompressionLayer;

pub async fn get_route(db: DatabaseConnection) -> Result<Router> {
    let session_config = SessionConfig::default();
    let session_config = set_cookie_key(session_config)?;
    let session_config = session_config.with_session_name("apikey");

    //https://github.com/AscendingCreations/AxumSession
    #[cfg(all(
        not(feature = "redis"),
        any(
            feature = "mysql-rustls",
            feature = "mysql-native-tls",
            feature = "sqlite-rustls",
            feature = "sqlite-native-tls",
            feature = "postgres-rustls",
            feature = "postgres-native-tls"
        )
    ))]
    let pool = session::get_pool(db.clone()).await?;
    #[cfg(feature = "redis")]
    let pool = session::get_pool().await?;

    // let session_middleware = session::get_session_middleware(session_storage)?;

    let session_storage =
        SessionStore::<dxp_axum_session::DbPool>::new(Some(pool.clone()), session_config).await?;

    let session_layer = SessionLayer::new(session_storage);

    let mut router = Router::new()
        .nest("/", routes::get_route().await?)
        .nest("/hot", routes::hot::get_route());

    router = router.nest(
        "/api",
        routes::api::get_route(db.clone(), session_layer).await?,
    );

    //go to http://127.0.0.1:8000/swagger
    #[cfg(feature = "swagger-ui")]
    {
        router = routes::swagger_ui::get_route(router, Some("/api/swagger.json"));
    }

    Ok(router.layer(CompressionLayer::new()).layer(Extension(db)))
}
