// ============================================================
// CHAPTER 8.3: STORING KEYS WITH ASSOCIATED VALUES IN HASH MAPS
// ============================================================
// Purpose: Look up data by meaningful key (any type) instead of
// numeric index. Use when you need key->value associations.
// ============================================================

use std::collections::HashMap;

fn main() {
    // --- 1. Creating a HashMap ---
    // Quick model: HashMap<K, V> stores key->value pairs on the heap.
    // We import HashMap explicitly because it is not in the prelude.
    let mut scores = HashMap::new();
    // `mut` is required because `insert` changes the map.
    // If `mut` is removed, compiler gives E0596 (cannot borrow as mutable).

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Important: map print/iteration order is arbitrary (not insertion order).

    // --- 2. Type homogeneity (HashMap<K, V>) ---
    // All keys must have the same type K.
    // All values must have the same type V.
    // From the inserts above, Rust infers:
    // scores: HashMap<String, i32>
    // Rule: types are inferred from first use, then locked in.
    //
    // Intentional type-mismatch probes (leave commented so this file runs):
    // scores.insert(42, 100);
    // ^ key type mismatch: expected String, found integer
    //
    // scores.insert(String::from("Green"), 3.14);
    // ^ value type mismatch: expected i32, found floating-point number

    println!("{:#?}", scores);

    // --- 3. Accessing values with get ---
    // `get(&key)` returns Option<&V>:
    // - Option because the key might be missing
    // - &V because the map keeps ownership of stored values
    //
    // `get` borrows the lookup key (&team_name) instead of taking ownership.
    // The key is only needed for hashing/comparison during lookup.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue's score: {:?}", score);

    // Common pattern for Copy values:
    // Option<&i32> -> Option<i32> via copied() -> i32 via unwrap_or(default)
    let score_value = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue's score (unwrapped): {}", score_value);

    // Missing key case: get returns None, unwrap_or provides fallback.
    let missing = scores.get(&String::from("Green")).copied().unwrap_or(0);
    println!("Green's score: {}", missing);

    // Principle recap so far:
    // 1) HashMap order is arbitrary; do not rely on insertion order.
    // 2) One key type + one value type per map.
    // 3) `get` returns Option<&V>; use copied().unwrap_or(...) for defaults.

    // --- 4. Iteration ---
    println!("\nAll scores:");
    for (team, scores) in &scores {
        println!("{}: {}", team, scores);
    }

    // --- 5. Ownership when inserting into a Hashmap ---
    // Quick model:
    // - For owned types like String, insert moves ownership into the map.
    // - For Copy types like i32, values are copied.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut preferences = HashMap::new();
    preferences.insert(field_name, field_value);
    println!("preferences: {:#?}", preferences);

    let hp = 100; // i32 is Copy
    let mut stats = HashMap::new();
    stats.insert(String::from("hp"), hp);
    println!("stats: {:#?}", stats);
    println!("hp is still usable after insert: {}", hp);

    // --- 6. Updating a value: insert overwrites for existing key ---
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    let old = scores2.insert(String::from("Blue"), 25);
    println!("old value returned by insert: {:?}", old);
    println!("scores2 after overwrite: {:#?}", scores2);
    println!("scores2 len after overwright: {}", scores2.len());

    let old_missing = scores2.insert(String::from("Yellow"), 50);
    println!("insert on new key returned: {:?}", old_missing);
    println!("scores2 now: {:#?}", scores2);

    // --- 7. Insert only if missing: entry(...).or_insert(...) ---
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    // Blue already exists, so value stays 10 (50 is ignored)
    let blue_ref = scores3.entry(String::from("Blue")).or_insert(50);
    println!("Blue via or_insert: {:#?}", blue_ref);

    // Yellow is missing, so it gets isnerted with 50.
    let yellow_ref = scores3.entry(String::from("Yellow")).or_insert(50);
    println!("Yellow via or_insert: {:#?}", yellow_ref);

    println!("scores3 after or_insert: {:#?}", scores3);

    // Bonus: or_insert returns &mut V, so you can update in place.
    *scores3.entry(String::from("Blue")).or_insert(0) += 5;
    println!("scores3 after + 5 to Blue: {:#?}", scores3);

    // --- 8. Updating based on old value (word counting) ---
    let text = "hello world wonderful world";

    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word counts: {:#?}", counts);

    // Intentional boundary probe
    // count += 1;
    // ^ won't compile; count is &mut i32, so you must dereference: *count += 1;
}
