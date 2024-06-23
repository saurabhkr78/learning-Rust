struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //here user1 is instance
    let user1 = User {
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true,
    };
//a placeholder {} which is replaced by the value of user1.username.
    println!("Username: {}", user1.username);
}
