// `String` is implemented as a wrapper around a vector of bytes.

#![allow(unused_variables, dead_code)]

fn main() {
    /*
     * Creating a new string
     */
    let mut _s = String::new();

    // `data` is type `&str`
    let data = "initial contents";

    // `to_string()` converts to owned `String`
    let s = data.to_string();

    // `data` still good. `to_string()` took a ref to `data`.
    println!("data: {data}");
    println!("first s: {s}");

    // Can also call `to_string()` on a literal
    let s = "initial contents".to_string();

    println!("second s: {s}");

    // `String::from` and `to_string()` do the same thing

    /*
     * Updating a String
     */

    // Use the `push_str` method to append a string slice
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    // `push` method takes a single char as param (note the single quote marks)
    s.push('l');
    println!("s is {s}");

    // Use the `+` operator or the `format!` macro to concatenate `String` values
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // The `+` operator RHS takes a ref (`&`). The `+` operator uses the `add` method
    let s3 = s1 + &s2; // `s1` moved here and can't be used anymore
    println!("s3: {s3}");

    // Use format! to combine strings. Better when using the `+` operator looks messy.
    // Also,`format!` uses refs and doesn't take param ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    /*
     * Indexing into Strings
     */
    // Invalid code. Rust strings don't support indexing.
    // let s1 = String::from("hi");
    // let h = s1[0];

    // How Rust stores strings in memory
}
