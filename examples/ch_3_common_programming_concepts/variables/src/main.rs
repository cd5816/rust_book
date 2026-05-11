fn main() {
    // Variables are **immutable** by default.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // ***Constants***
    // Constants are like immutable variables but with a few differences.
    // First, you can't use `mut` with constants. Second, it must have an explicit type. Third,
    // constants can be declared in any scope, including the global scope. Finally, constants have
    // to be set to a constant expression. It can't be a value computed at runtime.
    // Constants are useful for values that multiple parts of your application needs.

    const _THREE_HOURSE_IN_SECONDS: u32 = 60 * 60 * 3;

    // ***Shadowing**
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // `x` is 6 after the scope ends.

    println!("The value of x in the outer scope is: {x}");

    // We can change the variable's type when shadowing.
    // Using `mut` here: `let mut spaces = "   "` is an error.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces = {spaces}");
}
