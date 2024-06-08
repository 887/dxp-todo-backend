use anyhow::Result;

use poem::middleware::Compression;
use poem::{EndpointExt, IntoEndpoint, Route};
use sea_orm::DatabaseConnection;

mod api;
mod hot;
mod routes;
mod session;
mod swagger_ui;

pub async fn get_route(db: DatabaseConnection) -> Result<impl IntoEndpoint> {
    let session_storage = session::db_storage::get_db_storage(db.clone()).await?;
    let session_middleware = session::get_session_middleware(session_storage)?;

    let route = Route::new()
        .nest("/", routes::get_route().await?)
        .nest("/hot", hot::get_route()); //routers need to be nested

    //go to http://127.0.0.1:8000/swagger
    #[cfg(feature = "swagger-ui")]
    let route = route.nest("/swagger", swagger_ui::get_route(Some("/api/swagger.json")));

    let api_service = api::get_api_service("http://127.0.0.1:8000");
    let route = route.nest("/api", api::get_route(api_service, db.clone()).await?);

    Ok(route.with(session_middleware).with(Compression::new()))
}
