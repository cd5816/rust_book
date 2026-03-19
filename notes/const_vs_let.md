# `const` vs `let` in Rust

## When to use `const`

Use `const` when the value is a **true compile-time constant** — fixed for the entire lifetime of the program with semantic meaning as a named value.

- Value is known at compile time and never changes
- Needs to be available in **global scope** (outside any function)
- Value is **inlined** at every use site (no memory address exists)
- Represents a meaningful named constant (limits, sizes, fixed config)

## When to use `let`

- The value is computed at **runtime**, even if never changed
- The value only needs to exist within a **local scope**
- Binding the result of a function call, expression, or user input

## Key Differences

| | `const` | `let` |
|---|---|---|
| Scope | Any, including global | Local only |
| Type annotation | Required | Optional (inferred) |
| Mutability | Never mutable | Can be `mut` |
| Evaluated | Compile time | Runtime |
| Has memory address | No (inlined) | Yes (stack slot) |
| Shadowing | No | Yes |

## Practical Rule

> If you'd write `#define MAX 100` in C, use `const` in Rust. Everything else is `let`.

```rust
const MAX_HEALTH: u32 = 100;       // const — fixed program-wide value

fn main() {
    let player_health = MAX_HEALTH; // let — runtime binding
    let score = compute_score();    // let — result of a function
}
```

## Gotcha

`const` requires an explicit type annotation and only allows **const-evaluable expressions** — you can't call arbitrary functions or do heap allocation in a `const`.
