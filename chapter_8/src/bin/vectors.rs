// ============================================================
// CHAPTER 8.1: STORING LISTS OF VALUES WITH VECTORS
// ============================================================
// Purpose:
// - Vec<T> stores a growable list of values on the heap.
// - Use it when list length can change at runtime.
//
// Mental model:
// - The Vec value on the stack stores pointer + length + capacity.
// - Elements live contiguously on the heap.
// - Same-type elements means fixed element size, so index access is O(1).
//
// Quick peek (generics):
// - Vec<T> means "a vector of T".
// - Example: Vec<i32> is a vector where every element is i32.

fn main() {
    // ------------------------------------------------------------
    // 1) Create with vec![] (type inferred)
    // ------------------------------------------------------------
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);

    // ------------------------------------------------------------
    // 2) Create empty with Vec::new() (type annotation needed)
    // ------------------------------------------------------------
    let v2: Vec<i32> = Vec::new();
    println!("v2 = {:?}", v2);

    // ------------------------------------------------------------
    // 3) Update with push (requires mut)
    // ------------------------------------------------------------
    let mut v3 = Vec::new();
    v3.push(10);
    v3.push(20);
    v3.push(30);

    println!("v3 = {:#?}", v3);

    // Boundary test:
    // - This fails with E0596 because push needs mutable access.
    // - Fix: add `mut` to the binding.
    // let v4 = Vec::new();
    // v4.push(100);

    // ------------------------------------------------------------
    // 4) Read elements: indexing vs get
    // ------------------------------------------------------------
    // Indexing is zero-based.
    let v5 = vec![10, 20, 30, 40, 50];

    // &v[i] returns &T, and panics at runtime if out of bounds.
    let third = &v5[2];
    println!("Index method: third element = {}", third);

    // v.get(i) returns Option<&T> and never panics for bad indices.
    let third_opt = v5.get(2);
    match third_opt {
        Some(val) => println!(".get() method: third element = {}", val),
        None => println!(".get() method: no third element"),
    }

    let oob = v5.get(100);
    println!("v5.get(100) = {:?}", oob);

    // Boundary test:
    // - This compiles, but panics at runtime: index out of bounds.
    // - So this is a runtime failure, not a compile-time failure.
    // let oob_panic = &v5[100];
    // println!("This won't print: {}", oob_panic);

    // ------------------------------------------------------------
    // 5) Borrowing rules with vectors
    // ------------------------------------------------------------
    // Why Rust blocks this pattern:
    // - `first` immutably borrows an element.
    // - `push` needs mutable borrow of the vector.
    // - `push` may reallocate, which could invalidate `first`.
    // Result: E0502 (cannot borrow as mutable while immutably borrowed).
    //
    // Tiny visual:
    // old buffer ----first----> [1,2,3,4,5]
    // push may move vector to a new buffer and free old one.
    // first would dangle if Rust allowed this.
    // let mut v6 = vec![1, 2, 3, 4, 5];
    // let first = &v6[0];
    // v6.push(6);
    // v6[0] = 99;
    // println!("first = {}", first);

    // ------------------------------------------------------------
    // 6) Iterate over vector values
    // ------------------------------------------------------------
    // `for val in &v7` gives immutable references: &i32.
    let v7 = vec![100, 200, 300];
    println!("Immutable iteration:");
    for val in &v7 {
        println!(" {}", val);
    }

    // `for val in &mut v8` gives mutable references: &mut i32.
    // Quick peek (dereference `*`):
    // - val is a reference, *val is the actual value it points to.
    // - We need `*val += 10` to mutate the underlying element.
    let mut v8 = vec![1, 2, 3];
    println!("Before: {:#?}", v8);
    for val in &mut v8 {
        *val += 10;
    }
    println!("After: {:#?}", v8);

    // Boundary test:
    // - Cannot push while iterating with a borrow of the same vector.
    // - This also triggers E0502.
    // let mut v_iter_push = vec![1, 2, 3];
    // for val in &v_iter_push {
    //     v_iter_push.push(*val + 10);
    // }

    // Safe pattern A: read first, store additions separately, then push.
    let mut v9 = vec![1, 2, 3];
    let new_vals: Vec<i32> = v9.iter().map(|x| x + 10).collect();
    for new in new_vals {
        v9.push(new);
    }
    println!("v9 = {:#?}", v9);

    // Safe pattern B: freeze original length, then push based on old elements.
    // We only index 0..len (the original part), so this remains valid.
    let mut v9_2 = vec![1, 2, 3];
    let len = v9_2.len();
    for i in 0..len {
        v9_2.push(v9_2[i] + 10);
    }
    println!("v9_2 = {:#?}", v9_2);

    // Rule recap:
    // - Use `&v[i]` when out-of-bounds is impossible and should panic if wrong.
    // - Use `v.get(i)` when index may be invalid and you want Option handling.
    // - Avoid mutating vector structure while references/iteration borrows are active.

    // ------------------------------------------------------------
    // 7) Store multiple value shapes using an enum
    // ------------------------------------------------------------
    // Vectors require one concrete element type.
    // If you need "mixed" data, wrap variants in one enum type.
    // Then Vec<SpreadsheetCell> is still a single type at compile time.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Even though payloads differ, each element here is SpreadsheetCell.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // To read values back out, pattern-match each enum variant.
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }

    // Boundary test:
    // - This fails because vec![] requires one element type.
    // - Here we'd mix integer, string slice, and float directly.
    // let bad_row = vec![3, "blue", 10.12];

    // ------------------------------------------------------------
    // 8) Drop behavior
    // ------------------------------------------------------------
    // A vector is dropped when it goes out of scope.
    // Its elements are dropped too.
    // Rust prevents using references to elements after the vector drops.
    //
    // Example shape:
    // {
    //     let temp = vec![String::from("a"), String::from("b")];
    //     // use temp
    // } // temp + both Strings are dropped here
}
