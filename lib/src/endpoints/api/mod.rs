use poem::{
    get, handler,
    middleware::AddData,
    web::{Data, Html, Json},
    EndpointExt, IntoResponse, Route,
};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("world, {}!", name)),
            None => PlainText("world!".to_string()),
        }
    }

    #[oai(path = "/world", method = "get")]
    async fn world(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

//maybe use rapidoc instead of swagger
//https://rapidocweb.com/
//https://github.com/search?q=repo%3Apoem-web%2Fpoem%20swagger-ui&type=code

pub fn get_route(base_route: Route) -> Route {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://127.0.0.1:8000/api");
    let swagger_html = api_service.swagger_ui_html();

    // let spec = api_service.spec();

    //base_route = "/"  (Base route is Route.at("/"))
    //.nest here means base_route + nested

    let route = base_route
        .nest("api/", api_service) //this results in /api/
        .nest(
            "",
            Route::new()
                .nest("swagger/", get(swagger))
                .with(AddData::new(swagger_html)), //this result in /swagger/
        );

    //special: here we nest all the routes to "", so they are all accessible
    Route::new().nest("", route)

    //go to http://127.0.0.1:8000/swagger
}

#[handler]
pub fn swagger(Data(swagger_html): Data<&String>) -> impl IntoResponse {
    Html(swagger_html.to_owned())
}

#[handler]
pub fn spec(Data(spec): Data<&String>) -> impl IntoResponse {
    Json(spec.to_owned())
}
