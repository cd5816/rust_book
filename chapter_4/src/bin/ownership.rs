// Lesson summary
// Variables own their values; when they go out of scope, memory is freed
// Assigning or passing a `String` moves ownership - the original is invalidated
// Types like `i32` implement `Copy` and its value(s) is copied - no move
// Moving ownership in and out of functions works but is tedius

fn main() {
    {
        let s = "hello";
        println!("{s}");
    }
    // The code below errors. When a variable goes out of scope, it' gone.
    // println!("{s}");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    // In Rust, memory is freed when the owner goes out of scope.

    /*
    // the code below errors because I moved String ownership from `s1` to `s2`.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
    */

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s = String::from("hello");
    let s = takes_ownership(s);
    println!("{s}");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("'{s2}' has length {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn takes_ownership(s: String) -> String {
    s
}
