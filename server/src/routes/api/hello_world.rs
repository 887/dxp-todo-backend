use poem_openapi::{param::Query, payload::PlainText, OpenApi};
use tracing::trace;

pub struct HelloWorldApi;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

#[derive(poem_openapi::Tags)]
enum Tags {
    /// HelloWorld operations
    HelloWorld,
}

#[OpenApi]
impl HelloWorldApi {
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
}
