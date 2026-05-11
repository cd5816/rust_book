// Rust has three ownership rules:
// 1. Each Rust value has an owner
// 2. Each value can only have one owner at a time
// 3. The value is dropped whent the owner goes out of scope.

#![allow(unused)]
fn main() {
    // A scope is a program range where an item is valid.
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
    } // the scope is over. s is no longer valid

    // The String type manages data allocated on the heap. It's able to store an amount of text
    // unknown to us at compile time. Create a String like this:

    // The String type can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s = {s}");

    // ***Memory and Allocation***
    // A string literal (`&str`) is hardcoded into the final executable.
    // String::from requests the memory it needs.
    // How Rust manages memory: Memory is returned once the variable that owns it goes out of
    // scope. Rust calls the `drop` function when the variable goes out of scope. Rust calls it
    // automatically at the closing curly bracket.

    // **Variables and Data Interacting with Move
    let x = 5;
    let y = x;

    // `String` has three parts:
    // 1. A pointer to the first byte of string's heap-allocated buffer.
    // 2. Length - How many bytes the contents is using.
    // 3. Capacity - How many bytes `String` got from the allocator.
    // This "struct" is stored on the stack.
    let s1 = String::from("hello");
    // We copy the String struct on the stack, not the heap data.
    let s2 = s1;

    // Problem: Both `s1` and `s2` pointers point to the same memory location. And when they both
    // go out of scope, they'll both try to free the same memory. This is called a double free
    // error.
    // To ensure memory safety, Rust considers `s1` no longer valid after `let s2 = s1`. For
    // example, this code errors:
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1} world!"); // this errors
    // We say that s1 was moved into s2.

    // **Scope and Assignment**
    let mut s = String::from("hello");
    s = String::from("ahoy"); // Nothing is referring to s's original value "hello".
    println!("{s}, world!");

    // **Variables and Data Interacting with Clone**
    // `.clone()` makes an explicit duplicate of a value. "Duplicate" has different meanings based
    // on the type.
    // For `String`, this includes both the stack and heap values.
    // For example:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // If a type implements the `Copy` trait, like an integer, variables that use it don't move,
    // but are copied. That makes them, the variables, valid after assignment to another variable.
    // You'll notice that types like integers don't implement or need the `Drop` trait. But
    // `String`s do.

    // What types implement `Copy`? Generally, scaler values do. But types that require allocation,
    // like `String` does not. Here's some types that implement `Copy`:
    // - All integers like `u32`
    // - Boolean types `true` and `false`
    // - Floating-point types like `u64`
    // - `char`
    // - Tuples that contain types that also implement `Copy` like `(i32, i32)`, but not `(i32,
    // String)`

    // ***Ownership and Functions***
    // Passing a variable to a function will move or copy, like assignment.

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and is not valid here
    // println!("s = {s}"); // this doesn't work because s was moved into takes_ownership

    let x = 5; // x comes into scope

    makes_copy(x); // x does not move into the function because i32 implements the Copy trait. We
    // can use x afterward.
    println!("x is ok to use: {x}"); // this is ok!

    // ***Return Values and Scope***
    // Returning values can also transfer ownership

    // gives_ownership moves its return value into `s1`.
    let s1 = gives_ownership();

    let s2 = String::from("hello"); // `s2` comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back.
    // takes_and_gives_back return value is moved into s3.
    // At the end of the last }, s3 goes out of scope and is dropped. s2 was already moved out and
    // s1 goes out of scope and is dropped.

    // What if we want a function to use a value but not take ownership?
    // Rust lets us return multiple values via a tuple.

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // moving s1 into calculate_length. But calcuate_length
    // also returns the original string plus its length via a tuple.
    // This is a lot of work. But Rust has a feature for using a value without transferring
    // ownership.

    println!("The length of '{s2}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope.

    some_string // some_string is returned and moves out the calling function main.
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moved out to the calling function. main in this case.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
