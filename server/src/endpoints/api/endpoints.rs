use poem_openapi::{param::Query, payload::PlainText, OpenApi};

pub struct Api;

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
