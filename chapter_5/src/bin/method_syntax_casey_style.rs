struct Rectangle {
    width: u32,
    height: u32,
}

// Behavior as free function
fn rectangle_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn rectangle_can_hold(a: &Rectangle, b: &Rectangle) -> bool {
    a.width > b.width && a.height > b.height
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

    println!("rect1 area = {}", rectangle_area(&rect1));
    println!(
        "Can rect1 hold rect2? {}",
        rectangle_can_hold(&rect1, &rect2)
    );
    println!(
        "Can rect1 hold rect3? {}",
        rectangle_can_hold(&rect1, &rect3)
    );
}
