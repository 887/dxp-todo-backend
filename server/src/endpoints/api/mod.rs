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

pub fn get_route(server_url: &str) -> impl Endpoint {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(format!("{server_url}/api"));

    let specification = Spec {
        data: api_service.spec(),
    };

    let options = swagger::Options {
        url: Some("/api/swagger.json"),
        script: Some(&get_refresh_script()),
        ..Default::default()
    };

    Route::new()
        .at("/", api_service)
        .at(
            "/swagger.json",
            Route::new()
                .nest("", spec)
                .with(SetHeader::new().overriding("Content-Type", "application/json")),
        )
        .nest("/swagger", swagger::create_endpoint(options))
        .with(AddData::new(specification))

    //go to http://127.0.0.1:8000/swagger
}

fn get_refresh_script() -> &'static str {
    r#"
function refresh() {
    fetchAsync("../../hot").then((version_new) => {
        if (version != version_new) { 
            version = version_new;
            buildBundle();
        }
    });

    setTimeout(refresh, 1000);
}

// initial call
setTimeout(refresh, 1000);
    "#
}

#[handler]
pub fn spec(Data(spec): Data<&Spec>) -> impl IntoResponse {
    PlainText(spec.data.to_owned())
}
