use poem::handler;

#[handler]
pub fn index() -> String {
    println!("hello world! 1");

    "hello world 1!".to_string()
}
