use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

use poem::{session::SessionStorage, web::Data};
use poem_openapi::{
    param::Query,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, OpenApi,
};
use tracing::trace;

use serde_json::Value;

use crate::state::State;

pub struct AuthenticateApi;

//default is a tag
//https://github.com/poem-web/poem/discussions/44

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Authenticate operations
    Authenticate,
}

//https://github.com/poem-web/poem/issues/60
#[derive(ApiResponse)]
pub enum AuthenticationResult<T: ParseFromJSON + ToJSON> {
    #[oai(status = 200)]
    Some(Json<T>),
    //the generated session_id already existed. try again, this is a super unlikely case
    #[oai(status = 409)]
    Conflict,
    /// Returns when Session not found (None)
    #[oai(status = 401)]
    Forbidden,
}

#[OpenApi]
impl AuthenticateApi {
    /// Greetings
    #[oai(
        path = "/login",
        method = "put",
        tag = "Tags::Authenticate",
        operation_id = "authenticate"
    )]
    async fn greet(
        &self,
        state: Data<&State>,
        user_name: Query<String>,
        password: Query<String>,
        _device_info: Query<String>,
    ) -> poem::Result<AuthenticationResult<String>> {
        if password.0 != "password" {
            return Ok(AuthenticationResult::Forbidden);
        }
        let storage = &state.storage;

        let session_id = generate_session_id();
        let session = storage.load_session(&session_id).await?;
        if session.is_some() {
            return Ok(AuthenticationResult::Conflict);
        }

        let mut entries = BTreeMap::<String, Value>::default();

        //TODO: user_id UUID-v7 from db, check password, implement register etc,
        //TODO: insert session id + device info + user id in db
        // db.insert("device_info".to_string(), Value::String(device_info.0));
        entries.insert("user_name".to_string(), Value::String(user_name.0));

        storage.update_session(&session_id, &entries, None).await?;

        trace!("login");

        Ok(AuthenticationResult::Some(Json(session_id)))
    }
}

fn generate_session_id() -> String {
    let random_bytes = thread_rng().gen::<[u8; 32]>();
    BASE64_URL_SAFE_NO_PAD.encode(random_bytes)
}
