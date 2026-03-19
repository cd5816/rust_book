// =========================
// 6.3 Concise Control Flow with `if let`
// =========================
// Rust Book URL: https://doc.rust-lang.org/book/ch06-03-if-let.html
// Picture: `if let` is a shortcut for one pattern you care about.
// Use this when: you only care about one enum/Option pattern.
// Prefer `match` when: you need to handle many cases explicitly.

#[derive(Debug)]
enum Coin {
    Penny,
    Quarter(&'static str),
}

// `let...else` keeps the happy path flat:
// - If pattern matches, bindings (like `state`) are available below.
// - If pattern fails, `else` must diverge (return/break/continue/panic).
fn describe_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // Intentional boundary probe (leave commented so file compiles):
    // let Coin::Quarter(state) = coin else {
    //     println!("not a quarter");
    // };
    // E0308: `else` clause does not diverge.

    Some(format!("Quarter from {state}"))
}

fn main() {
    // Section 1: `match` and `if let` can express the same one-pattern check.
    // `if let` is shorter when other cases are "do nothing".
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("match: max = {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("if let: max = {max}")
    }

    // Section 2: `if let ... else` is one pattern + fallback behavior.
    let mut non_quarter_count = 0;

    let coin1 = Coin::Penny;
    if let Coin::Quarter(state) = coin1 {
        println!("quarter from {state}");
    } else {
        non_quarter_count += 1;
        println!("not a quarter, count = {non_quarter_count}");
    }

    let coin2 = Coin::Quarter("Alaska");
    if let Coin::Quarter(state) = coin2 {
        println!("quarter from {state}");
    } else {
        non_quarter_count += 1;
        println!("not a quarter, count = {non_quarter_count}");
    }

    // Section 3: `let...else` returns early on mismatch.
    println!("{:?}", describe_quarter(Coin::Penny));
    println!("{:?}", describe_quarter(Coin::Quarter("Alabama")));
}
