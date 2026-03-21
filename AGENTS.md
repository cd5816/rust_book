# Rust Book Learning Project

This project is for learning Rust by working through [The Rust Programming Language](https://doc.rust-lang.org/book/) (a.k.a. "the Rust book").

## Structure

Each chapter gets its own directory with a single Cargo project. Each section within a chapter gets its own file in `src/bin/`.

```
rust_book/
└── chapter_N/
    ├── Cargo.toml
    └── src/
        └── bin/
            ├── section_name.rs
            └── ...
```

Run a section with:
```
cargo run --bin section_name
```

## Creating a New Chapter

When starting a new chapter, use `--vcs none` to prevent Cargo from creating a nested `.git` folder:

```bash
cargo init chapter_N --vcs none
```

Then create the `src/bin/` directory for section files:

```bash
mkdir chapter_N/src/bin
```

**Reminder:** Always use `--vcs none` — the root repo already tracks everything.

## Teaching Style

- The user provides the Rust book URL for each lesson, and the assistant fetches that URL, reviews it, and bases the teaching plan on it.
- The **user types all the code themselves** — the assistant does not write code to files.
- The assistant teaches one concept at a time, has the user run it, and reacts to the output.
- Use a **purpose-first, probe-first style**:
  - Start with the concept's purpose: what problem it solves, what becomes easier/safer with it, and when to use it.
  - Give a tiny mental model before syntax.
  - Ask the user to predict output or compiler behavior before running.
  - Probe with small code experiments to test understanding.
  - Intentionally perturb or break code to discover boundaries via compiler errors.
  - Extract 1-2 concise rules from each probe.
- Use a quick "tool choice" check for each concept:
  - "Use this when..."
  - "Prefer something else when..."
- The assistant often asks the user to guess output before running.
- The assistant also asks the user to intentionally break code to see compiler errors — this is part of the learning.
- Notes and explanations stay in **code comments** inside the `.rs` files — no separate notes files.
- Keep explanations concise, focused, and concrete.

### ASCII Visual Command

- If the user asks for a visual, draw a compact ASCII diagram.
- Recommended user command format:
  - `draw me an ASCII visual for this: <concept sentence>`
- Keep visuals concrete and state-based (type definition -> variable slot -> valid runtime states).

Example visual style:

```text
Type definition (the menu of possibilities):
+----------------------+
| enum IpAddrKind {    |
|   V4,                |
|   V6,                |
| }                    |
+----------------------+

A variable of that type (one slot):
+----------------------+
| let ip: IpAddrKind   |
+----------------------+

At runtime, that slot holds EXACTLY ONE variant:

Case A:                    Case B:
+----------------------+   +----------------------+
| ip = IpAddrKind::V4  |   | ip = IpAddrKind::V6  |
+----------------------+   +----------------------+

Not allowed:
- both V4 and V6 at once
- any value outside {V4, V6}
```

## New Concept Peek Policy

- When a lesson uses a Rust concept not formally covered yet (for example: `impl`, `trait`, lifetimes, generics, macros), give a brief "quick peek" before continuing.
- Keep the peek short (2-5 lines): what it is, why it appears now, and one tiny example.
- Do not deep dive ahead of the book; just enough context so the learner is not blocked or confused.
- Continue the normal lesson flow after the peek.

## Preferred Teaching Workflow (Persistent)

- Teach one Rust concept at a time with short explanations.
- Default loop for each concept:
  - Picture: give a tiny visual model
  - Prediction: user guesses output/error first
  - Probe: run a minimal example
  - Perturb/Break: change one thing and observe compiler/runtime feedback
  - Principle: summarize the core rule in plain English
  - Pocket check: one quick verification question
- Keep lesson material in a single `.rs` file:
  - section headers
  - runnable examples
  - concise review comments
- Include intentional error examples (commented out) for compiler learning.
- End with a short recap quiz and detailed answers.
- Save quiz review as a markdown file in the chapter folder when requested.
- Assume this workflow by default in new sessions.

## Lesson File Template

- Use `chapter_4/src/bin/slices.rs` as the reference template for lesson structure:
  - clear lesson title comment block
  - numbered sections with concise comments
  - runnable examples for each concept
  - intentional error examples left commented out
  - a short exercise section near the end
  - section-by-section demos in `main`

## Progress

- **Chapter 3** — Common Programming Concepts (complete)
  - 3.1 Variables and Mutability → `chapter_3/src/bin/variables_and_mutability.rs`
  - 3.2 Data Types → `chapter_3/src/bin/data_types.rs`
  - 3.3 Functions → `chapter_3/src/bin/functions.rs`
  - 3.4 Comments — skipped (user already knows)
  - 3.5 Control Flow → `chapter_3/src/bin/control_flow.rs`

- **Chapter 4** — Ownership (complete)
  - 4.1 What Is Ownership? → `chapter_4/src/bin/ownership.rs`
  - 4.2 References and Borrowing → `chapter_4/src/bin/references.rs`
  - 4.3 The Slice Type → `chapter_4/src/bin/slices.rs`

- **Chapter 5** — Structs (complete)
  - 5.1 Defining and Instantiating Structs → `chapter_5/src/bin/defining_structs.rs` (complete)
  - 5.2 An Example Program Using Structs → `chapter_5/src/bin/example_structs.rs` (complete)
  - 5.3 Method Syntax → `chapter_5/src/bin/method_syntax.rs` (complete)

- **Chapter 6** — Enums (complete)
  - 6.1 Defining an Enum → `chapter_6/src/bin/defining_enums.rs` (complete)
    - covered: enum variants as one-of states (`IpAddrKind` mental model)
    - covered: variant namespacing (`EnumName::Variant`)
    - covered: data-carrying variants (`IpAddr::V4(String)`, `IpAddr::V6(String)`)
    - covered: mixed variant shapes (`Quit`, `Move { x, y }`, `Write(String)`, `ChangeColor(...)`)
    - covered: `Option<T>` basics (`Some`/`None`) and `Option<T>` vs `T` type safety
    - covered: exhaustive `match` boundary probe (`None` arm missing -> E0004)
    - covered: recap quiz + answers added in file comments
  - 6.2 The `match` Control Flow Construct → `chapter_6/src/bin/match.rs` (complete)
    - covered: basic enum matching and arm return expressions
    - covered: multi-line match arm block return (`1` vs `1;`)
    - covered: pattern binding (`Quarter(state)`) and using bound values
    - covered: `Option<T>` matching with exhaustive arms
    - covered: catch-all patterns (`other` vs `_`) and arm order boundary
    - covered: unreachable pattern warning when `_` is placed before specific arms
    - covered: recap quiz + answers added in file comments
  - 6.3 Concise Control Flow with `if let` and `let...else` → `chapter_6/src/bin/if_let.rs` (complete)
    - covered: `if let` as concise single-pattern alternative to `match`
    - covered: pattern binding in `if let` (`Some(max)` binds inner value)
    - covered: `if let ... else` as one-pattern + fallback flow
    - covered: `let...else` for "extract or return early" happy-path style
    - covered: boundary probe for `let...else` divergence requirement (`E0308`)
    - covered: purpose-first/tool-choice framing added to explanations and comments

- **Chapter 7** — Managing Growing Projects with Packages, Crates, and Modules (complete)
  - 7.1 Packages and Crates → `chapter_7/src/bin/packages_and_crates.rs` (complete)
    - covered: crate as smallest unit the compiler considers (binary vs. library)
    - covered: crate root (`src/main.rs`, `src/lib.rs`) and Cargo naming conventions
    - covered: package as a bundle of crates described by `Cargo.toml`
    - covered: at most one library crate per package; any number of binary crates
    - covered: `src/bin/*.rs` files each become a separate binary crate named after the file
    - covered: `cargo run` ambiguity error when multiple binary crates exist → must use `--bin`
  - 7.2 Defining Modules to Control Scope and Privacy → `chapter_7/src/bin/modules.rs` (complete)
    - covered: inline `mod {}` to group related code into named namespaces
    - covered: everything inside a module is private by default
    - covered: `pub mod` and `pub fn` to expose items to outside code
    - covered: privacy checked at every step of a path (E0603)
    - covered: sibling modules can see each other without `pub`; items inside siblings still need `pub`
    - covered: `super::` to navigate up one level in the module tree
    - covered: the implicit crate root module (`crate`) at the top of the module tree
    - covered: intentional error examples added (commented out) in the lesson file
  - 7.3 Paths for Referring to an Item in the Module Tree → `chapter_7/src/bin/paths.rs` (complete)
    - covered: absolute paths (`crate::module::item`) vs relative paths (`module::item`)
    - covered: prefer absolute for stability; relative when caller and target move together
    - covered: `super::` recap — child calling parent (deliver_order pattern)
    - covered: child modules can see ancestor items; privacy is a one-way gate
    - covered: `pub struct` makes struct visible but fields stay private (E0616)
    - covered: private fields require a constructor function (can't build from outside)
    - covered: `pub enum` makes all variants public automatically (opposite of structs)
    - covered: intentional error examples added (commented out) in the lesson file
  - 7.4 Bringing Paths into Scope with the `use` Keyword → `chapter_7/src/bin/use_keyword.rs` (complete)
    - covered: basic `use` as path shortcut to reduce repetition
    - covered: `use` is scope-local; bindings do not flow into child modules by default
    - covered: fixing child-module access with local `use` or `super::` path
    - covered: idiomatic imports — functions via parent module, structs/enums via full item path
    - covered: name-collision boundary (`Result` vs `Result`) and aliasing with `as`
    - covered: `pub use` re-export concept for shaping public API paths
    - covered: nested imports with `{}` and `self` (`std::io::{self, Write}`)
    - covered: trait method usage after import (`Write` with `writeln!`/`write_all`)
    - covered: glob import (`*`) tradeoffs and common test/prelude usage notes
    - covered: intentional error examples added (commented out) in the lesson file
  - 7.5 Separating Modules into Different Files → `chapter_7/separating_modules.md` (complete, conceptual)
    - covered: why and when to split modules across files as code grows
    - covered: `mod name;` declaration to load module code from files
    - covered: compiler file lookup rules for top-level modules and submodules
    - covered: modern file paths (`foo.rs`, `foo/bar.rs`) vs older `mod.rs` style
    - covered: `mod` vs `use` distinction (`mod` loads modules, `use` creates shortcuts)
    - covered: module vs file mental model (logical namespace vs physical storage)
    - covered: multi-file example layout using `src/lib.rs`, `src/front_of_house.rs`, and `src/front_of_house/hosting.rs`
    - note: section documented as conceptual walkthrough due to `src/bin/*.rs` lesson workflow

- **Chapter 8** — Common Collections (complete)
  - 8.1 Storing Lists of Values with Vectors → `chapter_8/src/bin/vectors.rs` (complete)
    - covered: `Vec<T>` purpose and memory model (stack metadata + contiguous heap elements)
    - covered: creating vectors with `Vec::new()` vs `vec![]` and type inference behavior
    - covered: updating vectors with `push` and mutability requirement (`E0596` boundary)
    - covered: reading elements via indexing (`&v[i]`) vs `get` (`Option<&T>`) and out-of-bounds behavior
    - covered: runtime panic for invalid indexing vs graceful `None` with `get`
    - covered: borrow checker conflict when holding element refs and mutating vector (`E0502`)
    - covered: why `push` can reallocate and invalidate references (reallocation mental model)
    - covered: iteration with `&v` and `&mut v`, including dereference operator `*` quick peek
    - covered: structural mutation during iteration boundary and safe alternatives
    - covered: storing mixed logical data via enum variants in `Vec<EnumType>`
    - covered: drop semantics for vectors and contained elements when scope ends
    - covered: purpose-first probes, perturb tests, and concise rule recaps added as comments in-file
  - 8.2 Storing UTF-8 Encoded Text with Strings → `chapter_8/src/bin/strings.rs` (complete)
    - covered: `String` vs `&str` ownership model and memory placement
    - covered: string literals as `&'static str` (stored in binary)
    - covered: idiomatic function parameter choice (`&str`) and `&String` -> `&str` coercion note
    - covered: creating strings with `String::new`, `String::from`, and `.to_string()`
    - covered: `String::len()` counts bytes; `.chars().count()` counts Unicode scalar values
    - covered: update operations with `push_str(&str)` and `push(char)`
    - covered: ownership behavior of `push_str` (borrows input, source remains usable)
    - covered: concatenation with `+` ownership behavior (`s1` moved, `s2` borrowed) and moved-value boundary (`E0382`)
    - covered: concatenation with `format!` returning `String` while keeping inputs usable
    - covered: why integer indexing on `String` is disallowed (`E0277`) and UTF-8 ambiguity/performance rationale
    - covered: string slicing with byte ranges, valid char-boundary slicing, and invalid-boundary runtime panic
    - covered: boundary-check tooling with `char_indices`, `is_char_boundary`, and safe char-first processing (`chars`, `take`, `collect`)
    - covered: byte iteration with `.bytes()` and byte-slice access with `.as_bytes()`
    - covered: grapheme-cluster note and why it requires external crates (`unicode-segmentation`)
    - covered: recap quiz completed; Q&A saved in comments at bottom of `strings.rs`
    - covered: lesson notes captured as comments in `strings.rs`
  - 8.3 Storing Keys with Associated Values in Hash Maps → `chapter_8/src/bin/hashmaps.rs` (complete)
    - covered: `HashMap<K, V>` purpose and key-based lookup mental model vs index-based vectors
    - covered: creating maps with `HashMap::new()` + `insert`, explicit `use std::collections::HashMap`
    - covered: mutability requirement for `insert` and boundary note (`E0596`)
    - covered: arbitrary print/iteration order (not insertion order)
    - covered: homogeneity and type inference lock-in from first insert (`HashMap<String, i32>`)
    - covered: type-mismatch boundary probe for wrong key/value types (`E0308`)
    - covered: access with `get(&key)` returning `Option<&V>`
    - covered: defaulting pattern for `Copy` values (`copied().unwrap_or(default)`)
    - covered: missing-key behavior (`None` -> fallback via `unwrap_or`)
    - covered: iteration with `for (k, v) in &map` and borrow-vs-move distinction in loops
    - covered: `insert` ownership behavior (owned types move, `Copy` types copy)
    - covered: overwrite behavior of `insert` for existing keys (`Some(old_value)` return, len unchanged)
    - covered: conditional insert with `entry(key).or_insert(default)`
    - covered: in-place mutation through returned mutable reference (`&mut V`, e.g. `*value += 1`)
    - covered: update-from-old-value pattern with word counting (`split_whitespace` + `entry`)
    - covered: recap quiz skipped by user

- **Chapter 9** — Error Handling (in progress)
  - 9.1 Unrecoverable Errors with `panic!` → `chapter_9/src/bin/panic.rs` (complete)
    - covered: direct panic with `panic!("crash and burn")` and source location reporting (`file:line:column`)
    - covered: out-of-bounds vector indexing panic (`v[99]`) and why Rust stops execution for safety
    - covered: reading panic output and tracing to user code location
    - covered: backtrace workflow with `RUST_BACKTRACE=1` and first-user-frame debugging rule
    - covered: unwind vs abort behavior and release-profile setting (`[profile.release] panic = 'abort'`)
  - 9.2 Recoverable Errors with `Result` → `chapter_9/src/bin/result.rs` (complete)
    - covered: purpose of `Result<T, E>` and matching on `Ok`/`Err`
    - covered: branching on `io::ErrorKind` (`NotFound`) and create-if-missing recovery path
    - covered: concise propagation with `?` and equivalent manual `match` mental model
    - covered: boundary probe E0277 (`?` in `main` returning `()`), then fix via `fn main() -> Result<(), Box<dyn Error>>`
    - covered: `?` with `Option` via `last_char_of_first_line` one-liner and expanded equivalent
    - covered: runtime path note for file APIs (relative paths resolve from current working directory)
  - 9.3 To `panic!` or Not to `panic!` → `chapter_9/src/bin/panic_or_not.rs` (in progress)
    - covered: decision framing (`panic!` for broken invariants/bugs, `Result` for expected failures)
    - covered: when `expect` is reasonable if human guarantees exceed compiler knowledge
    - covered: hardcoded IP parse with `expect` (`"127.0.0.1"`) and successful run
    - covered: boundary probe with invalid hardcoded IP (`"127.0.0.999"`) producing panic with context
    - next: guidelines for panic vs `Result` in app/library code and `Guess` validation type pattern
