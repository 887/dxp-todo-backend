use anyhow::Result;

use poem::{get, EndpointExt, IntoEndpoint, Route};
use sea_orm::DatabaseConnection;

mod api;
mod hot;
mod index;
mod session;
mod swagger_ui;

pub async fn get_route(db: DatabaseConnection) -> Result<impl IntoEndpoint> {
    let session_storage = session::get_db_storage(db.clone()).await?;
    let middleware = session::get_sever_session(session_storage)?;

    // let main_route = main_route;

    let route = Route::new()
        .at("/", get(index::index))
        .nest("/hot", hot::get_route()); //routers need to be nested

    //go to http://127.0.0.1:8000/swagger
    #[cfg(feature = "swagger-ui")]
    let route = route.nest("/swagger", swagger_ui::get_route(Some("/api/swagger.json")));

    let api_service = api::get_api_service("http://127.0.0.1:8000");
    let route = route.nest("/api", api::get_route(api_service));

    Ok(route.with(middleware))
}
