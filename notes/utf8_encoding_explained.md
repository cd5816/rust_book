# UTF-8 Encoding: How Characters Become Bytes

## The Two-Step Process

UTF-8 encoding happens in two steps:

1. **Unicode** assigns every character a unique number (code point)
2. **UTF-8** encodes that number into 1-4 bytes using specific bit-packing rules

## Example: Encoding 'З' (Cyrillic capital letter Ze)

### Step 1: Look up the Unicode code point

```
'З' = U+0417 = 1047 (decimal)
```

### Step 2: Apply UTF-8 encoding rules

For code points in the range U+0080 to U+07FF, UTF-8 uses a **2-byte template**:

```
Byte 1: 110xxxxx  (5 payload bits)
Byte 2: 10xxxxxx  (6 payload bits)
```

Convert 1047 to binary:

```
1047 = 10000010111  (11 bits)
```

Split into 5 + 6 bits:

```
10000010111
|--5--|--6--|
10000  010111
```

Stuff into the template:

```
Byte 1: 110 + 10000  = 11010000 = 208
Byte 2: 10  + 010111 = 10010111 = 151
```

Result: `'З'` is encoded as bytes `[208, 151]`.

## The UTF-8 Marker Bits

The `110` and `10` prefixes are **fixed markers** defined by the UTF-8 spec. They are not derived from the character — they signal the byte's role:

| Prefix | Meaning |
|--------|---------|
| `0xxxxxxx` | 1-byte character (ASCII, 0-127) |
| `110xxxxx` | First byte of a 2-byte character |
| `1110xxxx` | First byte of a 3-byte character |
| `11110xxx` | First byte of a 4-byte character |
| `10xxxxxx` | Continuation byte (middle/end of multi-byte char) |

This is how a UTF-8 decoder knows where characters start and end in a byte stream.

## Why This Matters in Rust

This marker system is exactly what `.is_char_boundary()` checks:

- Bytes starting with `10` (continuation) are **not** valid boundaries
- Bytes starting with `0`, `110`, `1110`, or `11110` **are** valid boundaries

That's why slicing a string at byte index 1 of `"Зд"` panics — index 1 is in the middle of `'З'`, pointing to a continuation byte.

## Full Encoding Table

| Byte count | Code point range | Byte 1 | Byte 2 | Byte 3 | Byte 4 |
|------------|------------------|--------|--------|--------|--------|
| 1 | U+0000 - U+007F | 0xxxxxxx | — | — | — |
| 2 | U+0080 - U+07FF | 110xxxxx | 10xxxxxx | — | — |
| 3 | U+0800 - U+FFFF | 1110xxxx | 10xxxxxx | 10xxxxxx | — |
| 4 | U+10000 - U+10FFFF | 11110xxx | 10xxxxxx | 10xxxxxx | 10xxxxxx |

## One-Liner Summary

UTF-8 takes a character's Unicode number, splits it into chunks, and wraps each chunk with marker bits that tell the decoder how to reassemble the character.
