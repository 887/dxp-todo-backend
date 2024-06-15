use std::collections::BTreeMap;

use poem::session::SessionStorage;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
}

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "X-API-Key", key_in = "query", checker = "api_checker")]
pub struct ApiKeySecurityScheme(ApiKey);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    req.ses(session_id)
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<User>::verify_with_key(api_key.key.as_str(), server_key).ok()
}

impl ApiKeySecurityScheme {
    pub async fn get_session(
        &self,
        storage: impl SessionStorage,
    ) -> Result<Option<BTreeMap<String, Value>>, poem::Error> {
        let api_key = &self.0.key;
        storage.load_session(api_key).await
    }
}
