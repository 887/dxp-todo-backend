use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

//https://github.com/codemountains/utoipa-example-with-axum/blob/main/src/main.rs

use crate::routes::api::authenticate;
use crate::routes::api::hello_world;
use crate::routes::api::session;
use crate::routes::api::test;
use crate::routes::api::todo;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "HelloWorld", description = "Hello world operations"),
        (name = "Authenticate", description = "Authenticate operations"),
        (name = "Test", description = "Test operations"),
        (name = "Todo", description = "Todo operations"),
    ),
    paths(
        todo::todo_put,
        test::test_put,
        authenticate::login,
        hello_world::hello,
        hello_world::greet,
    ),
    components(schemas(
        todo::Todo,
        test::Test,
        authenticate::AuthenticateApi,
        authenticate::AuthenticationResult,
    )),
   modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        );
    }
}
