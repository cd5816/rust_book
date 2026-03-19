# Chapter 7.5 — Separating Modules into Different Files

This section is mostly about *project organization*, not new runtime behavior.

In our `chapter_7/src/bin/*.rs` workflow, each file is its own binary crate, so the Rust book's multi-file `src/lib.rs` layout does not map cleanly to our lesson-file pattern. We are doing this section as a conceptual walkthrough.

## Purpose

As modules grow, keeping everything in one file gets hard to navigate.

Split module code into files so that:

- related code stays grouped
- each file stays focused and readable
- the file tree mirrors the module tree

## Core Rule

There are two ways to declare a module:

1. Inline module body:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

2. File-based module body:

```rust
mod front_of_house;
```

When Rust sees `mod front_of_house;`, it looks for that module's code in a file.

## Where Rust Looks

For a top-level module declared in crate root:

- `src/front_of_house.rs` (idiomatic modern style)
- `src/front_of_house/mod.rs` (older style, still supported)

For a child module `hosting` declared inside `front_of_house`:

- `src/front_of_house/hosting.rs` (idiomatic modern style)
- `src/front_of_house/hosting/mod.rs` (older style, still supported)

Do not define the same module with both styles at once (compiler error).

## Example Layout (from the book pattern)

```text
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

`src/lib.rs`:

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`src/front_of_house.rs`:

```rust
pub mod hosting;
```

`src/front_of_house/hosting.rs`:

```rust
pub fn add_to_waitlist() {}
```

## Important Distinction: `mod` vs `use`

- `mod` declares a module and tells the compiler where to load its code.
- `use` creates a shortcut name in a specific scope.

`use` does **not** control what files are compiled.

## Module vs File (Not the Same Thing)

A file is a disk location. A module is a namespace in Rust's module tree.

In the common file-based pattern, they line up 1:1, but conceptually they are different.

Example:

`src/main.rs`:

```rust
mod math;

fn main() {
    let result = math::add(2, 3);
    println!("{result}");
}
```

`src/math.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

What this means:

- `src/math.rs` is the file (physical storage)
- `math` is the module (logical namespace)
- `mod math;` wires the file contents into the `math` module in the crate tree

Quick map:

```text
Module tree (logical):          Files (physical):
crate                           src/
 └── math                        ├── main.rs
      └── add                    └── math.rs
```

More valid shapes:

1. One file can contain multiple modules (inline):

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

mod strings {
    pub fn greet() -> String { String::from("hi") }
}
```

2. One module can span multiple files via submodules:

```text
src/
├── lib.rs
├── math.rs
└── math/
    └── advanced.rs
```

`src/lib.rs`:

```rust
pub mod math;
```

`src/math.rs`:

```rust
pub mod advanced;
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

`src/math/advanced.rs`:

```rust
pub fn square(x: i32) -> i32 { x * x }
```

## Quick Rules to Remember

1. Declare each module with `mod` once in the module tree.
2. After declaration, refer to items by paths (`crate::...`, `super::...`, etc.).
3. Prefer the modern file style (`foo.rs`, `foo/bar.rs`) over repeated `mod.rs` files.
4. Keep module tree and folder tree aligned for readability.

## Why We Skipped a Full Hands-on Build Here

Our chapter workflow intentionally teaches each section in one `src/bin/<section>.rs` file.
Section 7.5 is specifically about splitting a crate across multiple files (typically `src/lib.rs`), so a full hands-on implementation would use a different crate structure than our current lesson format.
