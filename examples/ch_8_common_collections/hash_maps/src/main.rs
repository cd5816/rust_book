use std::collections::HashMap;

// Like vectors, hash maps store their data on the heap. Both the keys and values must have the same
// type.

fn main() {
    // Creating a New Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    // Managing Ownership in Hash Maps
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Bothe field_name and field_value are invalid at this point. Their ownership moved into the
    // hash map.

    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 10 doesn't exist anymore. It was overwritten by 25.

    println!("Scores: {scores:?}");

    // Adding a key and value only if a key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Scores: {scores:?}");

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace() outputs subslices: "hello" "world" "wonderful" "world"
    // A subslice looks like this: &text[0..5] -> "hello", &text[6..11] -> "world".
    // split_whitespace() is an iterator whose next() results look like this:
    // Some("hello")
    // Some("world")
    // ...
    // What an iterator is:
    // iterator = lazy stateful cursor (stateful means it "remembers" something)
    // It knows how to produce the next item. In this case with the next() method
    // An iterator is a concrete value, often a struct. Plus an implementation of the Iterator
    // trait.
    // An iterator is a data structure because it holds state. And it's an algorithm because it
    // defines how the next item is found.
    // If a type can answer: "what is next?" repeatedly, it's an iterator.
    for word in text.split_whitespace() {
        // count: &mut i32
        let count = map.entry(word).or_insert(0);
        // mutate the map value
        *count += 1;
    }

    println!("Map: {map:?}");

    // Hashing functions
    // HashMap by default uses a function called SipHash that provides resistance to DoS attacks
    // involving hash tables. It's not the fastes hashing algorithm. It trades performance for
    // security. You can switch hashing functions by specifying a different hasher.
}
