// =========================
// 6.2 The `match` Control Flow Construct
// =========================
// Rust Book URL: https://doc.rust-lang.org/book/ch06-02-match.html
// Picture: value goes through arms top-to-bottom and stops at first match.
// Rule: `match` must be exhaustive (all possible cases covered).

// -------------------------
// Section 1: Enum used for pattern matching
// -------------------------
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

// `Quarter` carries extra data; other variants do not.
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// `match` arms are expressions; matching arm's expression is returned.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // last expression in this block; do NOT add `;` or type becomes `()`
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // Pattern binding: `state` gets inner data from `Quarter(...)`.
            println!("State quarter from {state:?}");
            25
        }
    }
}

// -------------------------
// Section 2: `match` on Option<T>
// -------------------------
// `Some(i)` binds `i`; `None` has no inner value.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // Intentional break probe (leave commented so file compiles):
        // If `None` arm is removed, compiler gives E0004 (non-exhaustive patterns).
    }
}

// -------------------------
// Section 3: Catch-all patterns (`other` vs `_`)
// -------------------------
// Use `_` when you do not need the unmatched value.
fn dice_action(dice_roll: u8) {
    match dice_roll {
        3 => println!("add_fancy_hat"),
        7 => println!("remove_fancy_hat"),
        _ => println!("reroll"),
        // Boundary probe (leave commented): placing `_` first makes later arms unreachable.
        // Rust warns: unreachable pattern.
    }
}

fn main() {
    // ----- Section 1 demo: basic enum matching + bound data -----
    println!("Penny -> {}", value_in_cents(Coin::Penny));
    println!("Nickel -> {}", value_in_cents(Coin::Nickel));
    println!("Dime -> {}", value_in_cents(Coin::Dime));
    println!(
        "Quarter -> {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );
    println!(
        "Quarter -> {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    // ----- Section 2 demo: matching Option<T> -----
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six = {six:?}");
    println!("none = {none:?}");

    // ----- Section 3 demo: catch-all arm -----
    dice_action(3);
    dice_action(7);
    dice_action(9);
}

/*
=========================
6.2 Recap Quiz + Answers
=========================

Q1) In `match`, are arms checked top-to-bottom or bottom-to-top?
    What happens after the first matching arm is found?

A1) Arms are checked top-to-bottom. Once a pattern matches, Rust executes that
    arm and stops checking later arms.

Q2) Why does this fail in a function returning `u8`?
    Coin::Penny => {
        println!("Lucky penny!");
        1;
    }

A2) `1;` is a statement, so the block value becomes `()` (unit), not `u8`.
    The last expression must be `1` (no semicolon) to return `u8`.

Q3) What does `Coin::Quarter(state)` do in a `match` arm?

A3) It pattern-binds the inner payload of `Quarter(...)` into `state`, so the
    arm can use that value.

Q4) Why does this fail to compile?
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

A4) `match` must be exhaustive. `Option<i32>` has `Some` and `None`; `None`
    is missing, so Rust raises E0004 (non-exhaustive patterns).

Q5) When should you use `other => ...` vs `_ => ...` in a catch-all arm?

A5) Use `other => ...` when you need the unmatched value. Use `_ => ...` when
    you do not need it.

Q6) Why does putting `_ => ...` before specific arms (like `3 => ...`) cause
    a warning? What concept is Rust teaching?

A6) `_` matches every remaining value, so later specific arms become
    unreachable. Rust warns about unreachable patterns.
*/
