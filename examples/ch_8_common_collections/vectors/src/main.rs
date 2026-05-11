#![allow(unused_variables, dead_code)]

fn main() {
    // Creating an empty vector:
    let v: Vec<i32> = Vec::new();

    // Using a macro to create a vector; no type annotation needed.
    let v = vec![1, 2, 3];

    let v = vec![0, 1];
    read_slice(&v);

    // usize values are unsigned integers sized to the machine pointer width.
    // You can also do this:
    let u: &[usize] = &v;
    // Or this. This is an **explicit** slice ref:
    let u: &[_] = &v;

    // Updating a Vector:
    // Use `push` to add elements to a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v = {:?}", v); // Vec<T> implements Debug when `T` implements Debug.

    // Reading Vector Elements:
    // Two wasy to ref a vector value: i) indexing ii) `.get()` method
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
}

// Slice type def: a dynamically sized sequence of contiguous `T` values.
// Dynamically sized means the size is not known at compile time.
// `T` has no compile-time length, so it must be behind a pointer.
// A slice ref is a fat pointer: `&[ T]` pointer to first element + length.
fn read_slice(slice: &[usize]) {
    println!("{slice:?}");
}
