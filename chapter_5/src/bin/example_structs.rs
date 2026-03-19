// =========================
// 5.2 Example Program Using Structs
// =========================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Ownership choice guide:
// - Move (Rectangle) when the function should take ownership of the value.
// - Typical reasons:
//   - it stores the value for later (e.g., pushes into a Vec<Rectangle>)
//   - it consumes/transforms it into another owned value
//   - it needs to guarantee caller can't use old value afterward
// - Borrow (&Rectangle) when function just reads data.
// - Mutable borrow (&mut Rectangle) when function modifies in place but caller keeps ownership.
// Quick rule of thumb:
// - read => &T
// - mutate in place => &mut T
// - consume/take over => T

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    let area = area(&rect1);

    println!("area is: {area}");

    println!("rect1 is {rect1:#?}");
}
