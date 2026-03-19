# Chapter 4.3 Slices Quiz Review

## Quiz Questions

1. What is the key difference between returning `usize` (index) and returning `&str` (slice) for `first_word`?

2. Why does this fail at compile time?

```rust
let word = first_word(&s);
s.clear();
println!("{word}");
```

3. Which function signature is more idiomatic and why?

- `fn first_word(s: &String) -> &str`
- `fn first_word(s: &str) -> &str`

4. What does this produce, and why?

```rust
let a = [10, 20, 30, 40, 50];
let slice = &a[1..4];
```

5. Compile error or runtime panic?

```rust
let russian = "Здравствуйте";
let bad = &russian[0..1];
```

What exact rule is violated?

6. When should you choose:

- `&s[a..b]`
- `s.get(a..b)`

7. Why is scanning bytes for `b' '` safe even with UTF-8 strings?

---

## Detailed Answers

### 1) `usize` vs `&str`

Returning `usize` gives only a number (a byte index) that is meaningful only if the original string hasn’t changed.
Returning `&str` gives a slice reference tied to the original data, so Rust can enforce borrowing rules and prevent stale-index bugs.

---

### 2) Why `word` + `s.clear()` fails

`first_word(&s)` returns an immutable borrow (`word`) into `s`.
`clear()` needs a mutable borrow of `s`.
Rust forbids mutable and immutable borrows coexisting when the immutable one is still used later (`println!("{word}")`), so compilation fails with `E0502`.

---

### 3) More idiomatic signature

`fn first_word(s: &str) -> &str` is more idiomatic.

Why:

- More general API
- Works with `String` slices, `&String` (via coercion), and string literals (`&'static str`)
- No loss of functionality

---

### 4) Array slice result

```rust
[20, 30, 40]
```

Because range syntax `1..4` includes index `1` and excludes index `4`.

---

### 5) Panic vs compile error for `&russian[0..1]`

This is a **runtime panic**, not a compile error.

Rule violated: string slice indices must be at valid UTF-8 character boundaries.
`"З"` is 2 bytes in UTF-8, so `0..1` splits inside the character and panics at runtime.

---

### 6) `&s[a..b]` vs `s.get(a..b)`

Use `&s[a..b]` when you are certain boundaries are valid and want concise code.
Use `s.get(a..b)` when boundaries may be invalid (user input, computed ranges, mixed-language text) and you want safe handling with `Option<&str>` instead of panic.

---

### 7) Why searching for `b' '` is UTF-8-safe

Space is ASCII byte `0x20`.
UTF-8 multibyte sequences use lead and continuation byte ranges that do not include `0x20`.
So finding `b' '` in bytes identifies a real space character boundary, not a byte inside another character.

---

## Extra Notes

- String slicing uses **byte indices**, not character indices.
- `s.len()` returns byte length, not number of Unicode scalar values.
- `s.chars().count()` counts Unicode scalar values (often what humans think of as "characters," though grapheme clusters can still differ).
