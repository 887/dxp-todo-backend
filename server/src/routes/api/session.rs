use std::collections::BTreeMap;

use poem::{session::SessionStorage, web::Data};
use poem_openapi::{
    param::Query,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object, OpenApi,
};
use tracing::trace;

use serde_json::Value;

use crate::session::SessionStorageObject;

pub struct SessionApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Session operations
    Session,
}

#[derive(Object, Debug, PartialEq)]
pub struct UpdateSessionValue {
    pub entries: BTreeMap<String, Value>,
    pub expires: Option<u64>,
}

//https://github.com/poem-web/poem/issues/60
#[derive(ApiResponse)]
pub enum OptionalResponse<T: ParseFromJSON + ToJSON> {
    #[oai(status = 200)]
    Some(Json<T>),
    /// Returns when Session not found (None)
    #[oai(status = 404)]
    None,
}

fn frontend_session_id(session_id: Query<String>) -> String {
    ["fe_", &session_id.0].concat()
}

///TODO: Secure these endpoints so only the frontend can access them. These are for internal use only.
#[OpenApi]
impl SessionApi {
    /// Session
    #[oai(
        path = "/load_session",
        method = "get",
        tag = "Tags::Session",
        operation_id = "load_session"
    )]
    async fn load_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: Query<String>,
    ) -> poem::Result<OptionalResponse<BTreeMap<String, Value>>> {
        trace!("/load_session");
        let session_id = frontend_session_id(session_id);
        let entries = session.load_session(&session_id).await?;
        match entries {
            Some(entries) => Ok(OptionalResponse::Some(Json(entries))),
            None => Ok(OptionalResponse::None),
        }
    }

    #[oai(
        path = "/update_session",
        method = "put",
        tag = "Tags::Session",
        operation_id = "update_session"
    )]
    async fn update_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: Query<String>,
        value: Json<UpdateSessionValue>,
    ) -> poem::Result<()> {
        trace!("/update_session");
        let entries = value.0.entries;
        let expires = value.0.expires;

        let session_id = frontend_session_id(session_id);

        session
            .update_session(
                &session_id,
                &entries,
                expires.map(std::time::Duration::from_secs),
            )
            .await
    }

    #[oai(
        path = "/remove_session",
        method = "delete",
        tag = "Tags::Session",
        operation_id = "remove_session"
    )]
    async fn remove_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: Query<String>,
    ) -> poem::Result<()> {
        let session_id = frontend_session_id(session_id);

        trace!("/remove_session");
        session.remove_session(&session_id).await
    }
}
