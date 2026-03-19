# Chapter 5 Clarifying Notes

## 1) What is a struct and why use it?

A `struct` is a custom data type that groups related values under one name.

- It keeps related data together (for example: `username`, `email`, `active`, `sign_in_count`).
- It makes function signatures cleaner because you pass one value (`User`) instead of many separate variables.
- It improves readability and maintainability by modeling real concepts directly.
- It works naturally with Rust ownership and borrowing rules.

## 2) CPU-style view of a struct (with addresses)

Rust source:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Conceptual memory layout on a 64-bit machine (illustrative addresses):

```text
User value starts at: 0x0000_1000

STACK MEMORY
0x0000_1000: active = 0x01                (bool true, 1 byte)
0x0000_1001: padding = ??                 (compiler-inserted padding)
...
0x0000_1008: username.ptr = 0x0000_5000   (8 bytes)
0x0000_1010: username.len = 0x0000_0005   (8 bytes, "caleb")
0x0000_1018: username.cap = 0x0000_0010   (8 bytes)

0x0000_1020: email.ptr    = 0x0000_6000   (8 bytes)
0x0000_1028: email.len    = 0x0000_0011   (8 bytes)
0x0000_1030: email.cap    = 0x0000_0020   (8 bytes)

0x0000_1038: sign_in_count = 0x0000_0000_0000_002A  (u64 = 42)

HEAP MEMORY
0x0000_5000: 63 61 6C 65 62 ...   ("caleb" bytes)
0x0000_6000: 63 61 6C 65 62 40 ... (email bytes)
```

Key idea:

- `active` is stored at an address and has a byte value.
- `username` and `email` store pointer/length/capacity in the struct.
- The actual string bytes are stored on the heap at the pointer addresses.

Note:

- With Rust's default `repr(Rust)`, exact field ordering/padding is not a stable ABI guarantee.
- This is still a good mental model for understanding how data is represented in memory.
