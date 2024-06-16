use anyhow::Context;
use dxp_code_loc::code_loc;
use poem::{http::StatusCode, web::Data};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::{error::LogErrExt, state::State};

use super::security::ApiKeySecurityScheme;

pub struct TodoApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Test operations
    Todo,
}

//security
//https://github.com/poem-web/poem/blob/master/poem-openapi/tests/security_scheme.rs

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct Todo {
    pub test: String,
}

#[OpenApi]
impl TodoApi {
    #[oai(
        path = "/todo",
        method = "put",
        tag = "Tags::Todo",
        operation_id = "todo_put"
    )]
    async fn test(
        &self,
        state: Data<&State>,
        test: Json<Todo>,
        mut auth: ApiKeySecurityScheme,
    ) -> poem::Result<PlainText<String>> {
        trace!("/todo_put");
        let session = auth.session();

        //todo implement todo api
        state
            .db
            .ping()
            .await
            .context(code_loc!())
            .log_error()
            .map_err(|err| {
                poem::error::Error::from_string(
                    &format!("{}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;

        session.set("name", "name").await;

        session.update().await?;

        let t = test.0.test;
        Ok(PlainText(format!("todo_put:{}", t)))
    }
}
