// Rust uses snake_case for function and variable names.

fn main() {
    println!("Hello, world!");

    another_function(5);

    // ***Parameters***
    // Parameters are variables that are part of the function's signature. You must declare the
    // paramter's type.

    print_labeled_measurement(5, 'h');

    // ***Statements and Expressions***
    // Rust is an expression-based language.
    // Statements are instructions that perform an action but don't return a value.
    // Expressions evaluate to a value.

    let _y = 6; // Statement
    // Statements don't return values.

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // ***Functions with Return Values**
    // Declare a return value type after the arrow (`->`).
    // Return early from a function using the `return` keyword along with a value. Most functions
    // implicitly return the last expression.

    let x = five();
    println!("The value of x is {x}");

    let x = plus_one(5);
    println!("The value of x is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
