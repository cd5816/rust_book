// ============================================================
// Chapter 7.4 — Bringing Paths into Scope with the `use` Keyword
// ============================================================
// `use` creates a shortcut to a path, reducing repetition.
// Think of it like a symlink in a filesystem.
//
// Core rule: a `use` binding only exists in the scope where it is written.
// - Put `use` at crate root  -> crate-root code can use the shortcut.
// - Put `use` inside `main`  -> only `main` can use the shortcut.
// - Child modules do not automatically inherit parent `use` bindings.
// ============================================================

// Idiomatic for structs/enums: import the item directly.
// Then use the short type name (`HashMap`) in code.
use std::collections::HashMap;

// Chunk 3: `pub use` (re-export)
// - `use` creates a PRIVATE shortcut for this scope only.
// - `pub use` creates a PUBLIC shortcut, exposing that name as part
//   of your crate's API (mainly relevant for library crates).
//
// In a library crate, this would let external users write:
//   your_crate::hosting::add_to_waitlist()
// instead of:
//   your_crate::front_of_house::hosting::add_to_waitlist()
//
// We keep this in a bin file for learning, but the API-shaping benefit
// is most visible in `src/lib.rs`.

pub use crate::front_of_house::hosting;

// Chunk 4: nested `use` paths
// `use std::io::{self, Write};` is shorthand for these two lines:
//   use std::io;
//   use std::io::Write;
//
// `self` means "the module itself" (`std::io`), and `Write` is a trait
// inside that module.
use std::io::{self, Write};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist called");
        }
    }
}

mod customer {
    // `super` means "go up one module level".
    // Here, `super` points to crate root, where `hosting` is in scope
    // because of the crate-root `use` above.
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

fn main() {
    // --- Basic `use` shortcut for function calls ---
    // Because `hosting` was imported at crate root, we can use this short path
    // instead of `crate::front_of_house::hosting::add_to_waitlist()`.
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // This works because `customer` uses `super::hosting`.
    customer::eat_at_restaurant();

    // Re-export note:
    // `pub use` doesn't change behavior inside this binary.
    // It matters when OTHER crates import your library crate.

    // Intentional scope-break probe (leave commented):
    // If we move `use crate::front_of_house::hosting;` into `main`,
    // `super::hosting` inside `customer` will fail to resolve.

    // --- Struct import style (`HashMap`) ---
    // This demonstrates the idiomatic direct-item import for structs.

    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(5, 9);
    println!("map: {:?}", map);

    // --- Name collision probe (leave commented) ---
    // Two imports with the same final name (`Result`) collide:
    // use std::fmt::Result;
    // use std::io::Result;
    // → error[E0252]: `Result` is defined multiple times

    // Fix option A: import parent modules and qualify at use site:
    // use std::fmt;
    // use std::io;
    // fn a() -> fmt::Result { Ok(()) }
    // fn b() -> io::Result<()> { Ok(()) }

    // Fix option B: rename one import with `as`:
    // use std::fmt::Result;
    // use std::io::Result as IoResult;
    // fn a() -> Result { Ok(()) }
    // fn b() -> IoResult<()> { Ok(()) }

    // --- Nested path + trait method demo (`io::{self, Write}`) ---
    // `io::stdout()` comes from importing the `io` module via `self` above.
    // `writeln!` uses the `Write` trait implementation for Stdout.
    let mut stdout = io::stdout();

    // Direct trait method call version (same trait, lower-level API):
    // stdout.write_all(b"Write trait in action!\n").unwrap();

    // Macro convenience version (still powered by the `Write` trait):
    writeln!(stdout, "Write trait in action!").unwrap();

    // Glob import note (not used here):
    // use std::collections::*;
    // This imports ALL public names in that module. Useful in tests, but
    // usually avoided in app code because it can hide where names come from.
}
