// Summary:
// A reference lets you point to a value without taking ownership. The `&` symbol creates a ref.
// At any time, either one mutable ref or any number of immutable refs, never both at the same
// time.
// Refs **must always point** to valid data.

fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // This function "borrows" the value.
    // After the call to `calculate_length`, s1 is still valid because we passed a ref.
    println!("{s1} has length of {len}");

    change(&mut s1);
    println!("{s1}");

    let r = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
