use tracing::trace;

use crate::session::SessionType;

pub static SESSION_INDEX_COUNTER: &str = "index_counter";

pub async fn index(session: SessionType) -> String {
    let mut counter = session.get::<usize>(SESSION_INDEX_COUNTER).unwrap_or(0);
    counter += 1;

    session.set(SESSION_INDEX_COUNTER, counter);

    let hello = format!("hello world! {}", counter);
    trace!("{}", &hello);

    hello
}

pub async fn index2() -> String {
    let hello = "hello world!".to_string();
    trace!("{}", &hello);

    hello
}
