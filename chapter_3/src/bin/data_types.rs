fn main() {
    // -- Scaler types --, a single indivisible value

    let a: i32 = -42;
    let b: u32 = 42;
    println!("{a} {b}");

    // integers. Integers default to i32
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    println!("{decimal} {hex} {octal} {binary}");

    // floats. floats default to f64
    let x = 2.0;
    let y: f32 = 3.5;
    println!("{x} {y}");

    // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Rust truncates toward zero, not negative infinity.
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    // booleans
    let t = true;
    let f: bool = false;
    println!("{t} {f}");

    // characters
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // -- Compound types --, types that group multiple values together like tuples and arrays

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // two ways to access the values. First **destructuring**:
    let (x, y, z) = tup;
    println!("{x} {y} {z}");

    // second, **dot notation** by index:
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // arrays. All array elements **must be** the same type
    let a = [1, 2, 3, 4, 5];
    println!("{} {} {}", a[0], a[1], a[2]);

    let b: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type annotation: [type; length]
    let c = [3; 5]; // shorthand for [3, 3, 3, 3, 3]
    println!("{:?}", b);
    println!("{:?}", c);
}
