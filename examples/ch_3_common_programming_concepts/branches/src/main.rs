fn main() {
    // `if` Expressions
    let number = 3;

    // Blocks of code associated with are called arms; just like arms in match expressions.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    // The condition in the `if` expression ***must be*** a `bool`. The code below errors.
    // let number = 3;
    // if _number {
    //  println!("number was three") // error: expected `bool`, found integer.
    //}

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with `else if`
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using `if` in a `let` Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");
    // Remember: Blocks of code evaluate to its last expression. This means each arm's return value
    // must be the same type.
}
