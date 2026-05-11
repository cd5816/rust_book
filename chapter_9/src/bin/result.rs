// =========================
// Chapter 9.2: Recoverable Errors with `Result`
// =========================
// This lesson file compares two common styles:
// 1) `?` when you want to propagate errors upward.
// 2) `match` when you want branch-specific behavior.
//
// Path note:
// - Relative paths like "hello.txt" are resolved from the current working
//   directory (for `cargo run --bin result`, that is usually `chapter_9/`).

use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};

// -------------------------
// Section 1: `?` style (propagate errors)
// -------------------------
// Use this when the caller should decide how to handle failure.
// Mental model: on `Result`, `?` means:
// - `Ok(v)` => keep going with `v`
// - `Err(e)` => return early with `Err(e)`
fn read_with_question_mark(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

// -------------------------
// Section 2: `match` style (custom branch behavior)
// -------------------------
// Use this when error kind changes the action you want to take.
// Examples:
// - recover (create file when missing)
// - print extra context
// - return non-recoverable errors
fn read_with_custom_behavior(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!("{path} missing -> creating it with starter text");

            let mut new_file = File::create(path)?;
            new_file.write_all(b"Starter text created by custom branch.\n")?;
            File::open(path)?
        }
        Err(error) => {
            eprintln!("Could not open {path}: {error}");
            return Err(error);
        }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(bytes_read) => {
            println!("Custom branch read {bytes_read} bytes from {path}");
            Ok(contents)
        }
        Err(error) => {
            eprintln!("Could not read {path}: {error}");
            Err(error)
        }
    }
}

// -------------------------
// Section 3: `?` with `Option`
// -------------------------
// On `Option`, `?` means:
// - `Some(v)` => continue with `v`
// - `None` => return early with `None`
// This is the Option-version of error/absence propagation.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Expanded equivalent (same behavior as the one-liner above):
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     let first_line = match text.lines().next() {
//         Some(line) => line,
//         None => return None,
//     };
//
//     first_line.chars().last()
// }

// Boundary probes (intentionally commented so this file compiles):
// 1) `?` on `Result` requires a `Result`-compatible return type.
// fn main() {
//     let _v = read_with_question_mark("hello.txt")?; // E0277
// }
//
// 2) `?` does not auto-convert `Result` <-> `Option`.
// fn option_func() -> Option<String> {
//     let text = read_with_question_mark("hello.txt")?; // type mismatch
//     Some(text)
// }

fn main() -> Result<(), Box<dyn Error>> {
    // ----- Demo A: `?` case -----
    // If hello.txt is missing, `read_with_question_mark` returns Err.
    match read_with_question_mark("hello.txt") {
        Ok(text) => println!("`?` style succeeded:\n{text}"),
        Err(error) => println!("`?` style failed: {error:?}"),
    }

    // ----- Demo B: custom-branch case -----
    // If this file is missing, we create it and continue.
    match read_with_custom_behavior("custom_behavior_hello.txt") {
        Ok(text) => println!("custom behavior succeeded:\n{text}"),
        Err(error) => println!("custom behavior failed: {error:?}"),
    }

    // Boundary probe that now compiles because `main` returns `Result`:
    let value = read_with_question_mark("hello.txt")?;
    println!("{value}");
    // If `main` returned `()`, this line would fail with E0277.
    // `main -> Result<(), Box<dyn Error>>` makes `?` valid here.

    println!(
        "{:?}",
        last_char_of_first_line("Hello, world\nHow are you?")
    );
    println!("{:?}", last_char_of_first_line(""));
    println!("{:?}", last_char_of_first_line("\nhi"));

    Ok(())
}

// -------------------------
// Lesson notes recap (9.2)
// -------------------------
// - Prefer `Result<T, E>` when failure is expected and recoverable.
// - Use `?` to propagate errors concisely; use `match` for custom branches.
// - `unwrap()` and `expect(...)` panic on `Err`; `expect` is usually clearer.
// - `?` works only when the function return type is compatible.
// - In `main`, `fn main() -> Result<(), Box<dyn Error>>` allows `?` on Result.
