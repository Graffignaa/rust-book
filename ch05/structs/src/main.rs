struct User {
    //Fields
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Email is: {}", user1.email);

    // We can do this because the entire user1 instance is mut
    // Rust doesnt allow us to mark only certain fields as mutable
    user1.email = String::from("anotheremail@example.com");

    println!("Email changed to: {}", user1.email);

    let user2 = build_user(
        String::from("hello123"),
        String::from("hello123@example.com"),
    );

    // Verbose option
    //let user3 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //}; // Verbose option

    // Struct update syntax
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1 // Must come last, specifies any remaining fields get their corresponding
                // value from user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //Field init shorthand,  Can omit when struct field and function
        email,    //param have the same name
        sign_in_count: 1,
    }
}
