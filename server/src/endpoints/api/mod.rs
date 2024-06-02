use endpoints::Api;
use poem::{
    handler,
    middleware::{AddData, SetHeader},
    web::Data,
    Endpoint, EndpointExt, IntoResponse, Route,
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

pub fn get_route(api_service: OpenApiService<Api, ()>) -> impl Endpoint {
    let specification = Spec {
        data: api_service.spec(),
    };

    Route::new()
        .nest("/", api_service)
        .at(
            "/swagger.json",
            Route::new()
                .nest("", spec)
                .with(SetHeader::new().overriding("Content-Type", "application/json")),
        )
        .with(AddData::new(specification))

    //go to http://127.0.0.1:8000/swagger
}

pub fn get_api_service(server_url: &str) -> OpenApiService<Api, ()> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(format!("{server_url}/api"));
    api_service
}

#[handler]
pub fn spec(Data(spec): Data<&Spec>) -> impl IntoResponse {
    PlainText(spec.data.to_owned())
}
