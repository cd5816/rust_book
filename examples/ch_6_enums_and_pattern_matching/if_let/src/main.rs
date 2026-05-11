// if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
// Use if let when you want to match one pattern and ignore the rest.
#![allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max is {}", max),
        _ => (),
    }

    // You lose exhaustiveness checking when using if let.
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }

    // You can also use an else with if let. This is similar to _ in match.
    if let Some(max) = config_max {
        println!("Max is {}", max);
    } else {
        println!("No max found");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let state_description = describe_state_quarter(coin);
    println!("{state_description:?}");
}

/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
*/

/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
 */

// How Casey Muratori would write it.
/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state;
    match coin {
        Coin::Quarter(s) => state = s,
        _ => return None,
    }
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
*/

// Using let else syntax.
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
