# UTF-8: Quick Definition

UTF-8 is an **encoding scheme**: a set of rules for turning text characters into bytes (numbers) that computers can store and transmit.

## Core Idea

Every character in the world (letters, symbols, emoji, and more) has a unique number assigned by Unicode. UTF-8 defines how to represent those numbers using **1, 2, 3, or 4 bytes**.

## Variable Width (Key Property)

| Character | Unicode number | UTF-8 bytes |
|---|---|---|
| `H` | `U+0048` | 1 byte: `[72]` |
| `é` | `U+00E9` | 2 bytes: `[195, 169]` |
| `З` | `U+0417` | 2 bytes: `[208, 151]` |
| `你` | `U+4F60` | 3 bytes: `[228, 189, 160]` |
| `😀` | `U+1F600` | 4 bytes: `[240, 159, 152, 128]` |

## Why This Matters in Rust

Because characters are not all the same byte length, indexing into a string by a single integer is ambiguous. A byte index can land in the middle of a multi-byte character.

That is why Rust does **not** allow `string[0]` for `String`/`&str`.

In practice, Rust encourages thinking in three views:

- **Bytes**: raw UTF-8 bytes (how data is stored)
- **Unicode scalar values (`char`)**: decoded code points
- **Grapheme clusters**: what users often perceive as a single "letter"

## One-Line Summary

**UTF-8 is a variable-width Unicode encoding (1-4 bytes per character), which is why direct string indexing by integer is unsafe/ambiguous in Rust.**
