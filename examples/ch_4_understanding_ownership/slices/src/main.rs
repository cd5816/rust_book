fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    println!("'{word}'");

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("'{word}'");

    let a = [1, 2, 3, 4, 5];

    // Stores a ref to the first element and its length.
    let slice = &a[1..3];

    println!("{slice:?}");
    assert_eq!(slice, [2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("{:02x?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
