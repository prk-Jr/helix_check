#![allow(unused)]

use prkorm::*;

fn main() {
    let user = User::new(1, "Prakash".to_string(), "prakash@email.com".to_owned());
    let select_user_email = User::select_email().build();
    let select_where_user_id = User::select().where_id(5).build();

    println!("User: {:?}", user);
}
#[derive(Table, Debug)]
#[table_name("users")]
struct User {
    email: String,
    id: i32,
    username: String,
}

impl User {
    /// Creates a new [`User`].
    /// First arg is id: i32,
    /// Second arg is username: String,
    /// Final arg is email: String
    fn new(id: i32, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
}
