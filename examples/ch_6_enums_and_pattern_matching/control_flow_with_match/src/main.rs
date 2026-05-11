// match lets you compare a value against a series of patterns. And then execute code based on
// which pattern matches.
// Values could be: literal values, variable names, wildcards, etc.
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    fn value_in_cents(coin: Coin) -> u8 {
        // Similar to if/else statements, but more exhaustive. And if statements must evaluate to a boolean.
        match coin {
            // Match arms have two parts: a pattern and code to run. The matching arm expression is evaluated from top to bottom, and the first matching arm is executed. The return value from the matching arm is the return value from the entire match expression. Arms patterns must cover all variants.
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    //****************************************************** */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,           // The None pattern matches only when the value is None.
            Some(i) => Some(i + 1), // The Some(i) pattern matches only when the value is Some(i).
        } // The return value from the matching arm is the return value from the entire match expression.
    }

    let five = Some(5);
    let six = plus_one(five); // The plus_one function returns an Option<i32> because the input is an Option<i32>.
    println!("six: {six:?}");
    let none = plus_one(None);
    println!("none: {none:?}");

    //****************************************************** */
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You rolled a 3!"), // You can also call a function in a match arm. add_fancy_hat() would be a function call.
        7 => println!("You rolled a 7!"),
        other => println!("You rolled a {other}!"), // move_player(other) would be a function call. The catch-all arm must be the last arm.
    }
    // Rust also has a pattern when we don't need the value from the arm, we can use _ to ignore it.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max is {max}"),
        _ => (), // The () is the unit value, which is a value that does not have any value. It is used to ignore the value.
    }
}
