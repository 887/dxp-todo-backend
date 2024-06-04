use poem_openapi::{payload::PlainText, OpenApi};
use tracing::trace;

pub struct SessionApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Session operations
    Session,
}

#[OpenApi]
impl SessionApi {
    /// Session
    #[oai(
        path = "/session",
        method = "get",
        tag = "Tags::Session",
        operation_id = "session"
    )]
    async fn test(&self) -> PlainText<String> {
        trace!("/hello");
        PlainText("Hello, World!".to_string())
    }
}
