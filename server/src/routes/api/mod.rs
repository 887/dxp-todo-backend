use anyhow::Result;
use api_doc::ApiDoc;
use axum::{
    extract::Extension,
    handler::Handler,
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    session::{get_pool, DatabasePoolObject},
    state::State,
};

mod api_doc;
mod authenticate;
mod hello_world;
mod session;
mod test;
mod todo;

#[derive(Debug, Clone)]
struct Spec {
    pub json: String,
    pub yaml: String,
}

pub async fn get_route(db: DatabaseConnection) -> Result<Router> {
    let api_service = ApiDoc::openapi();
    let specification = Spec {
        json: api_service.to_json()?,
        yaml: api_service.to_yaml()?,
    };

    let session_storage = get_pool(db.clone()).await?;
    let session_storage_object = DatabasePoolObject {
        storage: session_storage.clone(),
    };

    let state = State::new(db, session_storage).await?;

    let app = Router::new()
        .route("/swagger.json", get(spec_json))
        .route("/swagger.yaml", get(spec_yaml))
        .nest("/", hello_world::routes())
        .nest("/", authenticate::routes())
        .nest("/", test::routes())
        .nest("/", todo::routes())
        .nest("/", session::routes())
        .layer(Extension(specification))
        .layer(Extension(session_storage_object))
        .layer(Extension(state));

    Ok(app)
}

pub fn get_api_service(server_url: &str) -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

async fn spec_json(Extension(spec): Extension<Spec>) -> impl IntoResponse {
    (
        [
            (CONTENT_TYPE, "application/json"),
            (CONTENT_DISPOSITION, "inline; filename=\"swagger.json\""),
        ],
        spec.json,
    )
}

async fn spec_yaml(Extension(spec): Extension<Spec>) -> impl IntoResponse {
    (
        [
            (CONTENT_TYPE, "application/x-yaml"),
            (CONTENT_DISPOSITION, "inline; filename=\"swagger.yaml\""),
        ],
        spec.yaml,
    )
}
