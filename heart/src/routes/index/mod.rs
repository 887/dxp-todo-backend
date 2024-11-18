use tracing::trace;

// use crate::session::SessionType;

// pub static SESSION_INDEX_COUNTER: &str = "index_counter";

// pub async fn index(session: SessionType) -> String {
//     let mut counter = session.get::<usize>(SESSION_INDEX_COUNTER).unwrap_or(0);
//     counter += 1;

//     session.set(SESSION_INDEX_COUNTER, counter);

//     let hello = format!(
//         "hello world! {}\r\nSession ID: {}",
//         counter,
//         session.get_session_id().inner()
//     );
//     trace!("{}", &hello);

//     hello
// }

pub async fn index() -> String {
    let hello = "hello world 1!".to_string();
    trace!("{}", &hello);

    hello
}

pub async fn index2() -> String {
    let hello = "hello world 2!".to_string();
    trace!("{}", &hello);

    hello
}
