// =========================
// 5.3 Method Syntax
// =========================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2
    }
}

// =========================
// Rules (5.3 quick review)
// =========================
// 1) Methods live in `impl Type { ... }` and use `self` as the first parameter.
//    - `&self` reads
//    - `&mut self` mutates
//    - `self` consumes/moves
//
// 2) Method-call syntax auto-adjusts the receiver:
//    - `rect.area()` can become `Rectangle::area(&rect)` behind the scenes.
//
// 3) In explicit associated-function style, you pass arguments exactly:
//    - `Rectangle::area(&rect)` works
//    - `Rectangle::area(rect)` fails for `fn area(&self)`
//
// 4) `&mut self` methods require a mutable binding at call site:
//    - `let mut rect = ...; rect.double_width();`
//
// 5) Field and method names can match; parentheses choose method:
//    - `rect.width`  -> field value (`u32`)
//    - `rect.width()` -> method result (`bool`)
//
// 6) Associated functions with no `self` are called with `::`:
//    - `Rectangle::square(3)`
//
// 7) Multiple `impl Rectangle` blocks are valid and equivalent to one block.

// ==========================================
// Review: Multiple `impl` Blocks (Same Type)
// ==========================================
// Rust allows multiple `impl Rectangle` blocks.
// This is equivalent to putting all methods in one `impl` block.
//
// Example:
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
//
// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }
//
// Notes:
// - Behavior is the same as a single `impl` block.
// - Choose the style that keeps code easiest to read.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let mut rect4 = Rectangle {
        width: 5,
        height: 10,
    };

    let _rect5 = Rectangle {
        width: 7,
        height: 8,
    };

    println!("rect1 area = {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("area via value syntax: {}", rect1.area());
    println!("area via explicit ref: {}", (&rect1).area());
    println!(
        "area via explicit function call: {}",
        Rectangle::area(&rect1)
    );
    println!("A: {}", rect1.area());
    println!("B: {}", (&rect1).area());
    println!("C: {}", Rectangle::area(&rect1));
    //println!("D: {}", Rectangle::area(rect1));

    let sq = Rectangle::square(3);
    println!("square = {sq:#?}, area = {}", sq.area());

    if rect1.width() {
        println!("rect1 has nonzero width; raw field value = {}", rect1.width);
    }

    println!("before: {:?}", rect4);
    rect4.double_width();
    println!("after: {:?}", rect4);

    // _rect5.double_width(); // fails: cannot borrow as mutable (_rect5 is not `mut`)

    println!("field width = {}", rect1.width);
    println!("method width() = {}", rect1.width());

    // Intentional error probe:
    // if rect1.width {
    //     println!("this should fail");
    // }
}
