use anyhow::Result;

use hello_world::HelloWorldApi;
use poem::{
    handler,
    middleware::{AddData, SetHeader},
    web::Data,
    Endpoint, EndpointExt, IntoResponse, Route,
};
use poem_openapi::{payload::PlainText, OpenApiService};
use sea_orm::DatabaseConnection;
use session::SessionApi;
use test::TestApi;
use todo::TodoApi;

use crate::{
    session::{storage, SessionStorageObject},
    state::State,
};

//combine multiple apis
//https://github.com/poem-web/poem/blob/master/examples/openapi/combined-apis/src/main.rs
pub type ApiService = OpenApiService<(HelloWorldApi, TestApi, SessionApi, TodoApi), ()>;

mod hello_world;
mod session;
mod test;
mod todo;

//maybe use rapidoc instead of swagger
//https://rapidocweb.com/
//https://github.com/search?q=repo%3Apoem-web%2Fpoem%20swagger-ui&type=code

#[derive(Debug, Clone)]
struct Spec {
    pub json: String,
    pub yaml: String,
}

pub async fn get_route(api_service: ApiService, db: DatabaseConnection) -> Result<impl Endpoint> {
    let specification = Spec {
        json: api_service.spec(),
        yaml: api_service.spec_yaml(),
    };

    let session_storage = storage::get_storage(db.clone()).await?;
    let session_storage_object = SessionStorageObject {
        storage: session_storage.clone(),
    };

    let state = State::new(db, session_storage).await?;

    let route = Route::new()
        .nest("/", api_service)
        .at(
            "/swagger.json",
            Route::new()
                .nest("", spec_json)
                .with(SetHeader::new().overriding("Content-Type", "application/json"))
                .with(
                    SetHeader::new()
                        .overriding("Content-Disposition", "inline; filename=\"swagger.json\""),
                ),
        )
        .at(
            "/swagger.yaml",
            Route::new()
                .nest("", spec_yaml)
                .with(SetHeader::new().overriding("Content-Type", "application/x-yaml"))
                .with(
                    SetHeader::new()
                        .overriding("Content-Disposition", "inline; filename=\"swagger.yaml\""),
                ),
        )
        .with(AddData::new(specification))
        .with(AddData::new(session_storage_object))
        .data(state);

    Ok(route)

    //go to http://127.0.0.1:8000/swagger
}

pub fn get_api_service(server_url: &str) -> ApiService {
    OpenApiService::new(
        (HelloWorldApi, TestApi, SessionApi, TodoApi),
        "Hello World",
        "1.0",
    )
    .server(format!("{server_url}/api"))
}

#[handler]
pub fn spec_json(Data(spec): Data<&Spec>) -> impl IntoResponse {
    PlainText(spec.json.to_owned())
}

#[handler]
pub fn spec_yaml(Data(spec): Data<&Spec>) -> impl IntoResponse {
    PlainText(spec.yaml.to_owned())
}
