// Every value in Rust is a certain data type.

fn main() {
    // Must tell the compiler what type for parse to work
    let _guess: i32 = "42".parse().expect("Not a number!");

    // ***Scaler Types***
    // A scaler type represents a single value. Rust has four: integers, floating-point numbers,
    // booleans, and characters.
    // ** Integer Types **:
    // An integer is a number without a fractional component. Signed numbers are stored using
    // two's compliment.
    // Integer types default to `i32`.

    // **Floating-Point Types**
    // Rust has two primitive floating-point types: `f32`, `f64`. Floating-point numbers are numbers with decimal. `f64` is the default floating-point type.
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // **Numeric Operations**

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    // Ineteger division truncates toward zero to the nearest integer.
    let _truncated = -5 / 3; // Result is -1

    // remainder
    let _remainder = 43 % 5; // Result is 3

    // **The Boolean Type**
    // Booleans are one byte in size.
    let _t = true;
    let _f: bool = false;

    // **The Character Type**
    // Rust's `char` type is the language's most primitive alphabetic type.
    // Specify `char` literals with a single quotation mark.
    // `char` is 4 bytes and represents a Unicode scaler value. That means it can represent more
    // than just ASCII.
    let _c = 'z';

    // ***Compound Types***
    // Compound types group multiple values into one type. Rust has two primitive compound types:
    // tuples and arrays.

    // **The Tuple Type**
    // Tuples have a fixed length and can values of different types.

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Use pattern matching to destruct a tuple value.
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let z = tup.2;

    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // A tuple with no values is called a *unit*, written `()`.

    // **The Array Type**
    // Unlike tuples, every array element must have the same type. Arrays also have a fixed length.

    let _a = [1, 2, 3, 4, 5];

    // Arrays are useful when you want data on the stack and you know the number of elements is
    // fixed and won't change.For example, if you were using the month names, you know month names
    // are fixed and won't change. An array is a good fit for this.

    // How to write an array's type: [T; N] where `T` is the element type and `N` is the number of
    // elements; it's length.
    // Array initialization shorthand:
    let a = [3; 5];
    println!("Array a = {a:?}");

    // **Array Element Access**
    // An array is a single chunk of memory with a fixed size that's stack allocated.

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first = {first}, second = {second}");

    // **Invalid Array Element Access**
    // Rust panics if you try to access an index value that is greater than or equal to the array's
    // length.
}
