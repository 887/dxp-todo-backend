use poem_openapi::{param::Query, payload::PlainText, OpenApi};
use tracing::trace;

pub struct Api;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

#[derive(poem_openapi::Tags)]
enum Tags {
    /// HelloWorld operations
    HelloWorld,
}

#[OpenApi]
impl Api {
    /// Say hello
    #[oai(path = "/hello", method = "get", tag = "Tags::HelloWorld")]
    async fn index(&self) -> PlainText<String> {
        trace!("/hello");
        PlainText("Hello, World!".to_string())
    }

    /// Greetings
    #[oai(path = "/greet", method = "get", tag = "Tags::HelloWorld")]
    async fn greet(&self, name: Query<Option<String>>) -> PlainText<String> {
        trace!("/greet");
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
