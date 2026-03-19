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

- **Chapter 6** — Enums (in progress)
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
    - next: 6.3 `if let`
