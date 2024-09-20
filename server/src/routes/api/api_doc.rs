use utoipa::OpenApi;

//https://github.com/codemountains/utoipa-example-with-axum/blob/main/src/main.rs

use crate::routes::api::test;
use crate::routes::api::todo;

#[derive(OpenApi)]
#[openapi(
    paths(
        todo::todo_put,
        test::test_put,
    ),
    components(schemas(
        todo::Todo,
        test::Test,
    )),
    tags((name = "Todo"))
)]
pub struct ApiDoc;
