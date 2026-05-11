// =========================
// 4.3 The Slice Type
// =========================
// This file keeps all slice examples from our lesson so you can review later.

// -------------------------
// Section 1: Fragile index approach (returns usize)
// -------------------------
// This version returns only the index where the first space appears.
// It works, but the index is just a number and can become stale if the String changes.
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// -------------------------
// Section 2: String slice return (&str) with &String parameter
// -------------------------
// Returning &str ties the result to the borrowed data.
fn first_word_slice_from_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// -------------------------
// Section 3: Idiomatic signature (&str -> &str)
// -------------------------
// This works with both String values and string literals.
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// -------------------------
// Section 4: Other slices (array slices)
// -------------------------
fn array_slice_demo() {
    let a = [10, 20, 30, 40, 50];
    let slice = &a[1..4];
    println!("array slice = {:?}", slice); // [20, 30, 40]
}

// -------------------------
// Section 5: UTF-8 boundary rule for string slices
// -------------------------
// String slices use byte indices, but indicies must land on valid UTF-8 boundaries.
fn utf8_boundary_demo() {
    let russian = "Здравствуйте"; // each Cyrillic char is multiple UTF-8 bytes.

    println!("bytes len = {}", russian.len());
    println!("chars count = {}", russian.chars().count());

    // Use &s[a..b] when you are certain indices are valid (and you want simple code).
    // Safe: first char "З" is 2 bytes, so 0..2 is a valid boundary.
    let first_char = &russian[0..2];
    println!("safe slice = {first_char}");

    // Intentional panic experiement
    // let bad_slice = &russian[0..1];
    // println!("bad slice = {bad_slice}");

    // Use s.get(a..b) when indices might be invalid (user input, computed offsets,
    // mixed-language text) and you want to handle failure gracefully instead of panicking.
    let maybe = russian.get(0..2);
    println!("{maybe:?}");
}

// -------------------------
// Section 6: .get() exercise
// -------------------------

fn first_word_safe(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return s.get(..i);
        }
    }

    Some(s)
}

fn main() {
    // ----- Section 1 demo -----
    let mut s_index_demo = String::from("hello world");
    let idx = first_word_index(&s_index_demo);
    println!("index = {idx}");
    println!("first word via index = {}", &s_index_demo[..idx]);

    // This mutation shows why a plain index can become meaningless.
    s_index_demo.clear();
    println!("after clear, stale index still = {idx}");

    // ----- Section 2 demo -----
    let s_slice_demo = String::from("hello world");
    let word = first_word_slice_from_string(&s_slice_demo);
    println!("first word (from &String param) = {word}");

    // Intentional borrow-checker experiment (leave commented so file compiles):
    // let mut s_error_demo = String::from("hello world");
    // let borrowed_word = first_word_slice_from_string(&s_error_demo);
    // s_error_demo.clear();
    // println!("still have word = {borrowed_word}");
    // If you uncomment this block, Rust errors with E0502 because you cannot
    // mutably borrow while an immutable borrow is still in use.

    // ----- Section 3 demo -----
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    let w1 = first_word_slice(&my_string); // &String coerces to &str
    let w2 = first_word_slice(&my_string[..]); // explicit String slice
    let w3 = first_word_slice(my_string_literal); // literal already is &str
    println!("{w1} {w2} {w3}");

    // ----- Section 4 demo -----
    array_slice_demo();

    // ----- Section 5 demo -----
    utf8_boundary_demo();

    // ----- Section 6 demo -----
    println!("{:?}", first_word_safe("hello world"));
    println!("{:?}", first_word_safe("hello"));
    println!("{:?}", first_word_safe("Здравствуйте мир"));
}
