#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Functions inside an impl block are called "associated functions".
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that aren't methods are used as constructors that return a new struct
    // instance.
    fn square(size: u32) -> Self {
        // Self is an alias for the type that's after the impl keyword; Rectangle here.
        Self {
            width: size,
            height: size,
        }
    }
}

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

    // dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("square's size is {:#?}", square);

    let square_area = square.area();
    println!("The square's area is {}", square_area);

    println!("rect1 is {rect1:#?}");

    println!("The rectangle's area is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    } else {
        println!("The rectangle has no width");
    }
}
