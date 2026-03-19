// =========================
// 6.1 Defining an Enum
// =========================
// Picture: an enum is one value chosen from a fixed set of variants.
// Rule: a value of an enum type can be EXACTLY ONE of its variants at runtime.

// -------------------------
// Section 1: Data-carrying enum variants
// -------------------------
// Each variant here carries a String payload.
// This is often cleaner than "enum + separate struct field for kind".
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// -------------------------
// Section 2: Mixed variant shapes in one enum
// -------------------------
// An enum can mix:
// - unit-like variants (no payload)
// - struct-like variants (named payload fields)
// - tuple-like variants (positional payload fields)
#[derive(Debug)]
enum Message {
    Quit,                       // unit-like variant (no data)
    Move { x: i32, y: i32 },    // struct-like variant (named fields)
    Write(String),              // tuple-like variant (one value)
    ChangeColor(i32, i32, i32), // tuple-like variant (three values)
}

// -------------------------
// Section 3: Enums can have methods via `impl`
// -------------------------
// Quick peek:
// - `impl Type { ... }` attaches methods to a type.
// - This works for enums just like structs.
#[derive(Debug)]
enum MessageCallDemo {
    Quit,
    Write(String),
}

impl MessageCallDemo {
    fn call(&self) {
        match self {
            MessageCallDemo::Quit => println!("call: Quit"),
            MessageCallDemo::Write(text) => println!("call: Write({text})"),
        }
    }
}

fn probe_c_enum_methods() {
    let m1 = MessageCallDemo::Quit;
    let m2 = MessageCallDemo::Write(String::from("hello"));
    m1.call();
    m2.call();
}

// `match` lets us branch by variant and read payload data.
fn show(ip: &IpAddr) {
    match ip {
        IpAddr::V4(addr) => println!("v4 address = {addr}"),
        IpAddr::V6(addr) => println!("v6 address = {addr}"),
    }
}

// Same idea on a more varied enum shape.
fn inspect_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {x}, {y}"),
        Message::Write(text) => println!("Write: {text}"),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: ({r}, {g}, {b})"),
    }
}

fn add_one(y: Option<i8>) -> Option<i8> {
    match y {
        Some(n) => Some(n + 1),
        _ => None, // Underscore `_` is a wildcard arm that means "anything not matched above."
                   // None is more explicit because it states intent directly, while `_` hides
                   // which case you're handling.
                   // Intentional boundary probe (do NOT leave this permanently):
                   // If you remove this arm, compiler error E0004 says `None` is not covered.
                   // That is Rust enforcing exhaustive handling for enum variants.
    }
}

fn main() {
    // ----- Section 1 demo: data-carrying enum variants -----
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    show(&home);
    show(&loopback);

    // ----- Section 2 demo: mixed variant shapes -----
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    // let bad = Message::Move(10, 20); wrong struct shape
    // ^ boundary rule: constructor shape must match variant shape exactly.
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 165, 0);

    inspect_message(&m1);
    inspect_message(&m2);
    inspect_message(&m3);
    inspect_message(&m4);

    // ----- Section 3 demo: Option<T> basics -----
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    // `None` needs a type annotation because it carries no inner value.

    println!("{some_number:?}");
    println!("{some_char:?}");
    println!("{absent_number:?}");

    // `Option<T>` and `T` are different types.
    // You must handle Some/None before doing T-only operations.
    // Intentional error probe (leave commented so file compiles):
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let _sum = x + y; // E0277: cannot add `Option<i8>` to `i8`

    // ----- Section 4 demo: handling Option with match -----
    println!("{:?}", add_one(Some(5)));
    println!("{:?}", add_one(None));

    // ----- Section 5 demo: enum method call -----
    probe_c_enum_methods();
}

// "Namespaced" means a name lives inside another name's scope,
// so you refer to it with a path.
//
// For enums, variants are namespaced under the enum type:
//
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// Use variants like:
// let home = IpAddrKind::V4;
//
// Think "folder path":
// - IpAddrKind = folder
// - V4 = item inside that folder
// - full path = IpAddrKind::V4
//
// Why this helps:
// - avoids name collisions
// - makes it clear where a variant comes from

/*
=========================
6.1 Recap Quiz + Answers
=========================

Q1) Why does this need a type annotation?
    let absent_number = None;

A1) Bare `None` has no inner value, so Rust cannot infer `T`.
    You must provide context like `let absent_number: Option<i32> = None;`.

Q2) Which is valid for `Message::Move { x: i32, y: i32 }`, and why?
    A) Message::Move(10, 20)
    B) Message::Move { x: 10, y: 20 }

A2) B is valid. `Move` is a struct-like variant, so construction must use
    braces with named fields.

Q3) Why does this fail?
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;

A3) `x` is `i8`, but `y` is `Option<i8>`. `+` is defined for `i8 + i8`, not
    `i8 + Option<i8>`. Rust requires handling `Some/None` first.

Q4) In `match opt { Some(n) => ..., _ => ... }`, what does `_` mean, and why
    is `None => ...` often clearer in `Option<T>` matches?

A4) `_` is a wildcard that matches any remaining case. In `Option<T>`, after
    handling `Some`, `_` effectively means `None`. Writing `None => ...`
    is usually clearer because it states intent explicitly.

Q5) What safety rule is Rust enforcing with:
    non-exhaustive patterns: `None` not covered

A5) `match` must be exhaustive: every possible enum variant must be handled.
*/
