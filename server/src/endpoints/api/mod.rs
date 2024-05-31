use endpoints::Api;
use poem::{
    get, handler, middleware::AddData, web::Data, Endpoint, EndpointExt, IntoResponse, Route,
};
use poem_openapi::{payload::PlainText, OpenApiService};

mod endpoints;

//maybe use rapidoc instead of swagger
//https://rapidocweb.com/
//https://github.com/search?q=repo%3Apoem-web%2Fpoem%20swagger-ui&type=code

#[derive(Debug, Clone)]
struct Spec {
    pub data: String,
}

pub fn get_route(server_url: &str) -> impl Endpoint {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(format!("{server_url}/api"));

    let specification = Spec {
        data: api_service.spec(),
    };

    Route::new()
        .at("/", api_service)
        .at("/swagger.json", get(spec))
        .nest("/swagger", swagger::create_endpoint("/api/swagger.json"))
        .with(AddData::new(specification))

    //go to http://127.0.0.1:8000/swagger
}

#[handler]
pub fn spec(Data(spec): Data<&Spec>) -> impl IntoResponse {
    PlainText(spec.data.to_owned())
}
