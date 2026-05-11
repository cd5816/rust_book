// Enums let you say one value is one of a possible set of values.
// An enum value can only be one of its variants.
// Option is an enum that says a value could be something (Some) or nothing (None).
// How the standard library defines the Option enum:
// enum Option<T> {
//  None,
//  Some<T>,
// }
// You have to convert an Option<T> to T before you can do T operations. For example: Option<i8> +
// i8 is an error.

// IpAddrKind is a custom data type.
#![allow(unused_variables, dead_code)]
#[derive(Debug)]
enum IpAddrKind {
    //V4(String),
    // Each variant can different types and amounts of data
    V4(u8, u8, u8, u8),
    V6(String),
}

// You can define methods on enums like structs using impl.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Instances of each IpAddrKind variant:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Associating data with an enum variant. No need for an extra struct:
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);

    // Example Option values
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
