// Rust has three loops: `loop`, `while`, and `for`

fn main() {
    // `loop` tells Rust to execute a code block forever until you manually tell it to stop.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // You can use `return` from inside the loop. But that exits the current function.

    let mut count = 0;
    // Label a loop using a single quote followed by a colon (`:`)
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!");

    // You can use a `while` loop to loop over collection values. But it's not ideal. Use a `for`
    // loop instead.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // The for loop increases code safety and eliminates the chance we go beyond the array's end. A
    // for loop can also be more performant because the index doesn't need to be compared to the
    // array's length at every iteration.
    for element in a {
        println!("the value is {element}");
    }

    // Using a for loop for a countdown:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
