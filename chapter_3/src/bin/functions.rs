// -- Functions --

// Order doesn't matter. You can call a function before or after its defined

// Paramaters require type annotations
fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

fn print_measurement(value: i32, unit: char) {
    println!("the measurement is: {value}{unit}");
}

// Statement - performs an action, returns nothing
// Expression - evaluates to a value
// In Rust, blocks `{}` are expressions. They evaluate to their last line

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    another_function(5);
    print_measurement(10, 'm');

    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of y is: {y}");

    let x = plus_one(5);
    println!("the value of x is: {x}");
}
