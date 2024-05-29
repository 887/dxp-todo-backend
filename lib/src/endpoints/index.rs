use poem::{handler, session::Session};

pub static SESSION_INDEX_COUNTER: &str = "index_counter";

#[handler]
pub fn index(session: &Session) -> String {
    let counter = session
        .get::<usize>(SESSION_INDEX_COUNTER)
        .map_or(0, |v| v + 1);

    session.set(SESSION_INDEX_COUNTER, counter);

    let hello = format!("hello world! {}", counter);
    println!("{}", &hello);

    hello.to_owned()
}
