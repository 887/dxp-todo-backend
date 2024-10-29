use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::Modify;
use utoipa::OpenApi;

use utoipauto::utoipauto;
//https://github.com/codemountains/utoipa-example-with-axum/blob/main/src/main.rs

// use crate::routes::api::authenticate;
// use crate::routes::api::hello_world;
// use crate::routes::api::session;
// use crate::routes::api::test;
// use crate::routes::api::todo;

//https://github.com/tokio-rs/axum/issues/50
//https://github.com/ProbablyClem/utoipauto

#[utoipauto(paths = "./heart/src")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "HelloWorld", description = "Hello world operations"),
        (name = "Authenticate", description = "Authenticate operations"),
        (name = "Test", description = "Test operations"),
        (name = "Todo", description = "Todo operations"),
        (name = "Session", description = "Session operations"),
    ),
    // paths(
    //     test::test_put,
    //     authenticate::login,
    //     hello_world::hello,
    //     hello_world::greet,
    //     session::load_session,
    //     session::update_session,
    //     session::remove_session,

    // ),
    // components(schemas(
    //     test::Test,
    //     authenticate::AuthenticateApi,
    //     authenticate::AuthenticationResult,
    // )),
    security(
        ("ApiKeyAuth" = [])
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

//https://docs.rs/utoipa/latest/utoipa/trait.Modify.html
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("apikey"))),
        );
    }
}
