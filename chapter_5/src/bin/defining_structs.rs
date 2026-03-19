/*
Chapter 5.1 - Defining and Instantiating Structs

A struct groups related values under one named type.
Struct instance fields are matched by field name, not by field order.
*/

// Core example struct used in sections 1-4.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user_verbose(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs and a unit-like struct used in section 5.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual; // unit-like struct

fn takes_color(c: Color) {
    println!("takes_color got: ({}, {}, {})", c.0, c.1, c.2);
}

// Unit-like structs are useful for:
// - marker types (type-level flags/state)
// - stateless services/strategies (like this logger)
// - trait/generic-heavy code where behavior matters more than data
// Why this logger pattern is useful:
// - No per-instance state, so fields add no value.
// - Still a concrete type you can implement traits on.
// - Easy to swap implementations without changing call sites.
// - Zero-sized type: behavior at near-zero storage cost.
// They are less common in domain models, where named-field structs are usually clearer.
struct ConsoleLogger;

trait Logger {
    fn log(&self, message: &str);
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[console] {message}");
    }
}

fn process_signup<L: Logger>(logger: &L, username: &str) {
    logger.log(&format!("created account for {username}"));
}

// Owns its data: simple and common for app/domain structs.
struct OwnedUser {
    username: String,
    email: String,
}

// Borrows data: requires lifetimes on referenced fields.
struct BorrowedUser<'a> {
    username: &'a str,
    email: &'a str,
}

fn main() {
    // 1) Define a struct and create an instance
    let user1 = User {
        active: true,
        username: String::from("caleb123"),
        email: String::from("caleb@example.com"),
        sign_in_count: 1,
    };

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);

    // Intentional error (leave commented): missing required field
    // let user2 = User {
    //     active: true,
    //     username: String::from("missing_email"),
    //     sign_in_count: 0,
    // };
    // Why this fails: every field must be provided when creating a struct instance.

    // 2) Mutating a field requires a mutable struct binding
    let mut user2 = User {
        active: true,
        username: String::from("before_change"),
        email: String::from("before@example.com"),
        sign_in_count: 1,
    };

    println!("before: {}", user2.email);
    user2.email = String::from("after@example.com");
    println!("after: {}", user2.email);

    // Intentional error (leave commented): cannot assign through immutable binding
    // let user3 = User {
    //     active: true,
    //     username: String::from("immutable_user"),
    //     email: String::from("immutable@example.com"),
    //     sign_in_count: 1,
    // };
    // user3.email = String::from("new@example.com"); // should fail

    // 3) Field init shorthand
    let user4 = build_user_verbose(
        String::from("verbose@example.com"),
        String::from("verbose_name"),
    );

    let user5 = build_user_shorthand(
        String::from("short@example.com"),
        String::from("short_name"),
    );

    println!("user4: {} / {}", user4.email, user4.username);
    println!("user5: {} / {}", user5.email, user5.username);

    // 4) Struct update syntax
    let user7 = User {
        active: true,
        username: String::from("original_name"),
        email: String::from("original@example.com"),
        sign_in_count: 42,
    };

    let user8 = User {
        email: String::from("new_email@example.com"),
        ..user7
    };

    println!("user8.email: {}", user8.email);
    println!("user8.username: {}", user8.username);
    println!("user8.active: {}", user8.active);
    println!("user8.sign_in_count: {}", user8.sign_in_count);

    // `email` was explicitly set on user8, so user7.email was not moved.
    // `active` is Copy, so it was copied rather than moved.
    println!("user7.email still usable: {}", user7.email);
    println!("user7.active still usable: {}", user7.active);

    // 5) Tuple structs and unit-like structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black red channel: {}", black.0);

    let Point(x, y, z) = origin;
    println!("origin destructured: ({x}, {y}, {z})");

    takes_color(black);

    let _subject = AlwaysEqual;
    println!("AlwaysEqual size: {}", std::mem::size_of::<AlwaysEqual>());

    let logger = ConsoleLogger;
    process_signup(&logger, "caleb123");
    println!(
        "ConsoleLogger size: {}",
        std::mem::size_of::<ConsoleLogger>()
    );

    // 6) Ownership of struct data: String vs &str

    // OwnedUser: owns String data
    let owned = OwnedUser {
        username: String::from("owned_name"),
        email: String::from("owned@example.com"),
    };
    println!("owned user: {} / {}", owned.username, owned.email);

    // BorrowedUser: borrows string slices with lifetime 'a
    let username = String::from("borrowed_name");
    let email = String::from("borrowed@example.com");

    let borrowed = BorrowedUser {
        username: &username,
        email: &email,
    };

    println!("borrowed user: {} / {}", borrowed.username, borrowed.email);

    // Intentional error (leave commented): missing lifetime specifier
    // struct BadUser {
    //     username: &str,
    //     email: &str,
    // }
}
