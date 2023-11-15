struct User {
    name: String,
    email: String,
}

fn build_user(email: String) -> User {
    User {
        name: String::from(""),
        email,
    }
}

fn main() {
    let user1 = build_user(String::from("jpcercal@gmail.com"));
    
    // The syntax .. specifies that the remaining fields not explicitly set
    // should have the same value as the fields in the given instance.
    let user2 = User {
        name: String::from("João"),
        ..user1
    };

    println!("Hello, {:?} ({:?})!", user2.name, user2.email);

    // Note that the struct update syntax uses = like an assignment; this is
    // because it moves the data, just as we saw in the "What Is Ownership?"
    // section. In this example, we can no longer use user1 after creating user2
    // because the String in the username field of user1 was moved into user2.
    // If we had given user2 new String values for both email and username, and
    // thus only used the active and sign_in_count values from user1, then user1
    // would still be valid after creating user2. The types of active and
    // sign_in_count are types that implement the Copy trait, so the behavior we
    // discussed in the “Copying vs. Moving Out of a Vector” section would
    // apply.
    // println!("Hello, {:?} ({:?})!", user1.name, user1.email); !!!would not work!!!
}
