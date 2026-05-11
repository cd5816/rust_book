// -- Control Flow --

// `if` expressions
// Rust has no truthy or falsy values like JavaScript or C. The condition must be a bool
// Rust stops at the first `true` condition and skips the rest
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }

    // `if` is a Rust expression. You can use it in a `let` statement
    // an "arm" is a branch of an `if` expression. An arm is each possible path code can take

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {number}");

    // -- Loops --

    let mut counter = 0;

    // `loop` is a Rust keyword. It runs forever until you `break`. It's similar to `while (true)`.
    // But you can return a value out of it via `break`.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is: {result}");

    // `while` loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // `for` loop
    let mut a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // when you only need indices
    for i in 0..a.len() {
        println!("index: {i}");
    }

    // read-only borrow
    for (i, number) in a.iter().enumerate() {
        println!("index: {i}, value: {number}");
    }

    // mutable borrow
    for (i, number) in a.iter_mut().enumerate() {
        *number += i;
    }

    // take ownership by value
    let v = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];

    // v is moved by into_iter() so you can't use v here anymore
    for (i, item) in v.into_iter().enumerate() {
        println!("index: {i}, item: {item}");
    }

    // `for` is preferred over `while` with an index because with `for` there is not risk of going
    // out of bounds. `for` loops through elements only within the array.

    // iterating over a range
    // 1..4 creates a range from 1 up to, but not including 4.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println! {"LIFTOFF!"};
}
