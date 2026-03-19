// ============================================================
// Chapter 7.1 — Packages and Crates
// ============================================================
// Key vocabulary:
// - Crate: smallest unit of code the compiler considers at once.
//   Two forms: binary crate (has main) and library crate (no main).
// - Crate root: the source file the compiler starts from
//   (src/main.rs for binary, src/lib.rs for library).
// - Package: a bundle of crates described by Cargo.toml.
//   At most 1 library crate, any number of binary crates.
//
// Cargo conventions:
//   src/main.rs        → binary crate named after the package
//   src/lib.rs         → library crate named after the package
//   src/bin/<name>.rs  → binary crate named <name>
//
// If multiple binaries exist, `cargo run` requires --bin <name>.
// ============================================================

fn main() {
    println!("I am another binary crate from src/bin/packages_and_crates.rs")
}
