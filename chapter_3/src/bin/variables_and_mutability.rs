fn main() {
    // --- Mutable Variables ---
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // --- Constants ---
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // --- Shadowing ---
    let x = 5;
    let x = x + 1; // x is now 6

    {
        let x = x * 2; // x is 12 in this inner scope
        println!("inner scope x: {x}");
    }

    println!("outer scope x: {x}"); // back to 6

    // Shadowing can change the type of a variable
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize — valid via shadowing
    println!("spaces: {spaces}");
}

/*
 * Shadowing is useful for:
 *
 * 1. Type conversion without new names
 *    let x = "5";                    // &str
 *    let x: i32 = x.parse().unwrap(); // i32 — same name, new type
 *    // No need for x_str, x_int, etc.
 *
 * 2. Sequential transformations
 *    let data = read_file();
 *    let data = parse(data);
 *    let data = validate(data);
 *    // Each step uses the same conceptual name
 *
 * 3. Making variables immutable after setup
 *    let mut x = String::new();
 *    x.push_str("ready");      // actual mutation during setup
 *    let x = x;                 // now immutable
 *
 * 4. Fine-grained scoping (as shown above)
 *    - Inner scope can shadow without affecting outer scope
 *    - Useful for temporary calculations
 *
 * The key benefit: same concept, different representation — you don't
 * pollute the namespace with x1, x2, x_as_string, etc.
 */
