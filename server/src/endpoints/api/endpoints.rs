use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    Object, OpenApi,
};
use serde::{Deserialize, Serialize};
use tracing::trace;

pub struct Api;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

//combine multiple apis
//https://github.com/poem-web/poem/blob/master/examples/openapi/combined-apis/src/main.rs

#[derive(poem_openapi::Tags)]
enum Tags {
    /// HelloWorld operations
    HelloWorld,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct Test {
    pub test: String,
}

#[OpenApi]
impl Api {
    /// Say hello
    #[oai(
        path = "/hello",
        method = "get",
        tag = "Tags::HelloWorld",
        operation_id = "hello"
    )]
    async fn index(&self) -> PlainText<String> {
        trace!("/hello");
        PlainText("Hello, World!".to_string())
    }

    /// Greetings
    #[oai(
        path = "/greet",
        method = "get",
        tag = "Tags::HelloWorld",
        operation_id = "greet"
    )]
    async fn greet(&self, name: Query<Option<String>>) -> PlainText<String> {
        trace!("/greet");
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(
        path = "/test",
        method = "put",
        tag = "Tags::HelloWorld",
        operation_id = "test"
    )]
    async fn test(&self, test: Json<Test>) -> PlainText<String> {
        trace!("/test");
        let t = test.0.test;
        PlainText(format!("test:{}", t))
    }
}
