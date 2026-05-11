// A struct or structure is a custom data type. It lets you package related values that make up a
// group. Structs let you create custom types that are meaningful for your domain.
// A struct groups related data under a single name — make that name describe the whole, not its parts.
#![allow(dead_code, unused_variables)]
#[derive(Debug)]
struct User {
    // { field name, field type}
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
// Use when implementing a trait on a type but don't want to store data on that type.
struct AlwaysEqual;

fn main() {
    // The entire instance must be mutable to mutate fields.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Can't move fields that don't implement Copy like String.
    let user2 = User {
        email: String::from("another@example.com"),
        username: user1.username.clone(),
        ..user1
    };

    println!("user1 = {:#?}, user2 = {:#?}", user1, user2);

    let black = Color(0, 0, 0);
    let first_color = black.0;
    println!("first color = {first_color}");

    let origin = Point(0, 0, 0);

    // Destructure a tuple struct
    let Point(x, y, z) = origin;
    println!("{x}, {y}, {z}");

    // Instance of AlwaysEqual
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}
