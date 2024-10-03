mod api_session;

mod object;
use axum_session::Key;
use axum_session::SessionConfig;
pub use object::*;

mod abstractions;
pub use abstractions::*;

use std::env;

use anyhow::Context;
use anyhow::Result;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

pub fn set_cookie_key(session_config: SessionConfig) -> Result<SessionConfig, anyhow::Error> {
    let cookie_key = env::var("COOKIE_KEY").context("COOKIE_KEY is not set")?;

    let cookie_key_bytes =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
            .decode(cookie_key)
            .context("COOKIE_KEY not base64")?;

    Ok(session_config.with_key(Key::from(&cookie_key_bytes)))
}

// pub fn get_session_middleware<S>(storage: S) -> Result<ServerSession<S>>
// where
//     S: SessionStorage,
// {
//     let cookie_key = env::var("COOKIE_KEY").context("COOKIE_KEY is not set")?;

//     let cookie_key_bytes =
//         engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
//             .decode(cookie_key)
//             .context("COOKIE_KEY not base64")?;

//     let cookie_key = CookieKey::from(&cookie_key_bytes);

//     Ok(ServerSession::new(
//         CookieConfig::signed(cookie_key),
//         storage,
//     ))
// }
