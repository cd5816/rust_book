# UTF-8 View vs Byte View (`&str` vs `&[u8]`)

The underlying memory is always just bytes. `&str` and `&[u8]` are two different views of the same data.

Example string: `"hé"`

```text
Index:   0    1    2
Bytes:  68   C3   A9      (hex)
        'h'  └─ 'é' ─┘    ('é' is 2 bytes in UTF-8)
```

## View 1: `&[u8]` (raw bytes)

- You see raw values: `[0x68, 0xC3, 0xA9]`
- Indexing is by byte position
- No text meaning is enforced

## View 2: `&str` (UTF-8 text)

- Rust interprets those bytes as valid Unicode text: `"h"`, `"é"`
- Slicing must land on UTF-8 boundaries
- `&s[0..1]` is valid (`"h"`)
- `&s[1..2]` is invalid (middle of `"é"`)
- `&s[1..3]` is valid (`"é"`)

So:

- `&[u8]` answers: "What are the raw bytes?"
- `&str` answers: "Interpret these bytes as valid UTF-8 text."
