use poem::Route;
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
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

pub fn get_route(route: Route) -> Route {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://127.0.0.1:8000/api");
    let ui = api_service.swagger_ui();
    route.nest("/api", api_service).nest("/swagger", ui)

    //go to http://127.0.0.1:8000/swagger
}
