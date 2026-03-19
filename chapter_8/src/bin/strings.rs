// ============================================================
// CHAPTER 8.2: STORING UTF-8 ENCODED TEXT WITH STRINGS
// ============================================================
// Concept 1: String vs &str
// Purpose:
// - Rust separates owned text from borrowed text views.
// - This makes ownership and mutation explicit and safe.
//
// Mental model:
// - String: owned, growable UTF-8 text on the heap.
// - &str: borrowed slice view into UTF-8 bytes stored elsewhere.
//
// Two string types in Rust:
// - String: owned, growable, heap-allocated, UTF-8
// - &str: borrowed slice, read-only view into UTF-8 bytes
//
// String literals ("hello") are &'static str - baked into binary.
// Use &str for function params to accept both String refs and literals.
//
// Quick peek (deref coercion):
// - &String can coerce to &str automatically.
// - This is why fn takes_name(name: &str) accepts both literals and &String.
//
// Tool choice:
// - Use String when you need ownership or mutation/growth.
// - Use &str for read-only borrowing and flexible function parameters.
//
// Principle recap:
// - Same printed text can come from different types and memory locations.
// - "hello" => &'static str (binary data); String::from("hello") => owned heap data.
// --- WHAT IS AN ITERATOR? ---
// An iterator is an object that produces a sequence of values one at a time, on demand.
// Think of it like a vending machine with a "next" button.

fn main() {
    // String literal - type is &'static str
    let literal = "hello";

    // Owned String - heap allocated, we own it
    let owned = String::from("hello");

    println!("literal: {}", literal);
    println!("owned: {}", owned);

    // ------------------------------------------------------------
    // Concept 2: Creating new String values
    // ------------------------------------------------------------
    // Purpose:
    // - Build owned, heap-allocated text you can pass around and mutate later.
    //
    // Three common constructors:
    // - String::new()         -> empty String
    // - String::from("...")   -> owned copy from string literal/slice
    // - "...".to_string()     -> same result as String::from for literals
    //
    // Quick peek (Display trait):
    // - to_string() works on any type implementing Display.
    // - For string literals, to_string() and String::from(...) are equivalent.
    //
    // Tool choice:
    // - Prefer String::new() when starting empty and filling later.
    // - Prefer String::from(...) or to_string() when you already have text.
    //
    // Rule:
    // - String::len() returns byte length, not human character count.
    // - For Unicode scalar count, use .chars().count().
    let s1 = String::new(); //empty
    let s2 = String::from("hello"); //from literal
    let s3 = "hello".to_string(); // also from literal

    println!("s1: '{}' (len {})", s1, s1.len());
    println!("s2: '{}' (len {})", s2, s2.len());
    println!("s3: '{}' (len {})", s3, s3.len());

    // Use `.chars().count() to count visible characters, not .len()`
    let s = "Здравствуйте";
    println!("bytes = {}", s.len()); // 24 bytes
    println!("chars = {}", s.chars().count()); // 12 characters

    // ------------------------------------------------------------
    // Concept 3: Updating a String
    // ------------------------------------------------------------
    // Purpose:
    // - String is growable, so we can append text after creation.
    //
    // Two append methods:
    // - push_str(&str): append a string slice (possibly many chars)
    // - push(char):     append exactly one Unicode scalar value
    //
    // Ownership behavior:
    // - push_str borrows its input; it does not take ownership.
    // - So the original source (like `s7`) remains usable after append.
    //
    // Boundary reminder:
    // - Both push_str and push mutate the String, so the String binding must be `mut`.
    // - "x" is &str, while 'x' is char (different types, different methods).
    let mut s4 = String::from("foo");
    s4.push_str("bar"); // append &str
    println!("s4 after push_str: {}", s4);

    let mut s5 = String::from("lo");
    s5.push('l'); // append one char
    println!("s5 after push: {}", s5);

    // push_str borrows; it doesn't take ownership
    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("s6 = {}", s6);
    println!("s7 is still usable: {}", s7);

    // ------------------------------------------------------------
    // Concept 4: Concatenation with `+`
    // ------------------------------------------------------------
    // Purpose:
    // - Combine existing strings into a new owned String.
    //
    // Key signature idea behind `+`:
    // - Trait-level (generic): add(self, rhs: Rhs) -> Self::Output
    // - String impl (concrete here): add(self, rhs: &str) -> String
    // - Left side (`self`) is moved (ownership taken).
    // - Right side is borrowed as `&str`.
    //
    // Quick peek (deref coercion):
    // - `&String` can coerce to `&str`, so `s1 + &s2` compiles.
    //
    // Efficiency model:
    // - Result can reuse/extend the left String's heap buffer.
    // - This avoids copying both strings into a brand-new buffer first.
    //
    // Rule recap:
    // - After `let s3 = s1 + &s2;`, `s1` is moved and cannot be used.
    // - `s2` is still valid because it was only borrowed.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    println!("s2 is still valid: {}", s2);

    // Boundary test (uncomment to see E0382: use of moved value `s1`):
    // println!("s1 = {}", s1);

    // ------------------------------------------------------------
    // Concept 5: Concatenation with `format!`
    // ------------------------------------------------------------
    // Purpose:
    // - Build combined strings in a readable way, especially with many parts.
    //
    // Behavior:
    // - format! works like println! formatting, but returns a String.
    // - It borrows inputs, so original strings remain usable afterward.
    //
    // Tool choice:
    // - `+` is okay for simple two-part concatenation when move semantics are fine.
    // - `format!` is usually clearer for 3+ parts and avoids ownership surprises.
    //
    // Rule recap:
    // - format!(...) -> String
    // - Arguments are not moved by default in this common usage pattern.
    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let game = format!("{t1}-{t2}-{t3}");
    println!("game = {}", game);

    // All three originals are still valid
    println!("t1 = {}, t2 = {}, t3 = {}", t1, t2, t3);

    // ------------------------------------------------------------
    // Concept 6: Why indexing into String is not allowed
    // ------------------------------------------------------------
    // Purpose:
    // - Prevent ambiguous/bug-prone "character indexing" with UTF-8 text.
    //
    // Core idea:
    // - String stores UTF-8 bytes, not fixed-width characters.
    // - One visible letter may use multiple bytes.
    // - So `s[i]` by integer is unclear (byte? char? grapheme?).
    //
    // Rust choice:
    // - Disallow integer indexing on str/String at compile time (E0277).
    // - Require explicit APIs: .bytes(), .chars(), or byte-range slices.
    //
    // Performance note:
    // - O(1) indexing by "character" is not generally possible for UTF-8.
    // - Rust avoids implying constant-time char indexing where it cannot guarantee it.
    //
    // Rule recap:
    // - .len() returns bytes.
    // - .chars().count() returns Unicode scalar values.
    // - For "nth character", use .chars().nth(n) (not string indexing).
    let hello = String::from("Здравствуйте");

    // Boundary test (uncomment to see E0277):
    // - `str` cannot be indexed by integer.
    // let h = hello[0];

    // Bytes vs chars
    println!(
        "'{}' has {} bytes but {} chars, 3rd char = {:?}",
        hello,
        hello.len(),
        hello.chars().count(),
        hello.chars().nth(2)
    );

    // ------------------------------------------------------------
    // Concept 7: Slicing strings with ranges
    // ------------------------------------------------------------
    // Purpose:
    // - Build a &str view into part of a string when you know byte bounds.
    //
    // Core rule:
    // - String slices use BYTE ranges, not character indexes.
    // - Range endpoints must be valid UTF-8 char boundaries.
    //
    // Safety behavior:
    // - Valid boundary -> returns &str slice.
    // - Invalid boundary (inside a code point) -> runtime panic.
    //
    // Tool choice:
    // - Prefer .chars() for general text processing.
    // - Use byte slicing when boundaries are known/validated.
    let hello2 = "Здравствуйте";

    // valid slice: 0..4 covers exactly 2 Cyrillic chars (2 bytes each)
    let slice = &hello2[0..4];
    println!("slice = '{}'", slice);

    // BOUNDARY TEST - slice that cuts through a character (runtime panic!)
    // let bad_slice = &hello2[0..1];
    // println!("bad_slice = '{}'", bad_slice);

    // Practical boundary checks before slicing:
    // 1) Use `.char_indices()` to see safe byte offsets for each char.

    for (i, c) in hello2.char_indices() {
        println!("byte {}: '{}'", i, c);
    }

    // 2) Use `.is_char_boundary(n)` to validate a specific byte index.
    println!(
        "{}, {},{}",
        hello2.is_char_boundary(0), // true
        hello2.is_char_boundary(1), // false
        hello2.is_char_boundary(2)  // true
    );

    // 3) Use `.chars()` for safe character iteration (no manual indexing).
    println!("chars:");
    for c in hello2.chars() {
        println!(" {}", c);
    }

    // If you need the first three characters as a String
    let first_three: String = hello2.chars().take(3).collect();
    println!("First three chars = {}", first_three);
}
