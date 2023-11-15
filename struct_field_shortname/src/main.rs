struct User {
    username: String,
}

fn build_user(username: String) -> User {
    // Here, we’re creating a new instance of the User struct, which has a field
    // named username. We want to set the username field’s value to the value in
    // the username parameter of the build_user function. Because the username
    // field and the username parameter have the same name, we only need to
    // write username rather than username: username.
    User {
        username,
    }
}

fn main() {
    let user1 = User {
        username: String::from("João"),
    };
    let user2 = build_user(String::from("Paulo"));

    println!("Hello, {}!", user1.username);
    println!("Hello, {}!", user2.username);
}
