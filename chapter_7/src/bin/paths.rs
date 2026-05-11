// ============================================================
// Chapter 7.3 — Paths for Referring to an Item in the Module Tree
// ============================================================
// Two ways to reach any item:
//   Absolute path: starts with `crate::` (like "/" in a filesystem)
//   Relative path: starts from the current module (like "./" )
// ============================================================

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist called");
        }
    }
}

fn deliver_order() {
    println!("deliver_order called");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("cook_order called");
    }
}

fn main() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // Not allowed. Can't set private field outside its module.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Appetizers ordered: {:?} and {:?}", order1, order2);

    // ---- Intentional error examples (uncomment to see compiler errors) ----

    // ERROR 1: private field access — can't set seasonal_fruit from outside
    // meal.seasonal_fruit = String::from("blueberries");
    // → error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private

    // ERROR 2: direct construction of struct with private field — not possible
    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("Rye"),
    //     seasonal_fruit: String::from("peaches"),
    // };
    // → error[E0451]: field `seasonal_fruit` of struct `Breakfast` is private

    // ERROR 3: accessing private module — hosting without pub
    // (change `pub mod hosting` to `mod hosting` above, then try:)
    // crate::front_of_house::hosting::add_to_waitlist();
    // → error[E0603]: module `hosting` is private
}

// ============================================================
// Key rules from 7.3:
// 1) Absolute path: `crate::module::item` — starts from the crate root.
//    Prefer this by default for stability when moving code.
// 2) Relative path: `module::item` — starts from the current module.
//    Use when caller and target will always move together.
// 3) `super::` — go up one level (like `..` in a filesystem).
//    Use when a child module needs to call something in its parent.
// 4) Child modules can see ancestor items. Parents cannot see into
//    children without `pub`. Privacy is a one-way gate.
// 5) `pub struct` makes the struct visible, but fields stay PRIVATE.
//    Each field needs its own `pub`. Private fields require a
//    constructor function (e.g., Breakfast::summer()).
// 6) `pub enum` makes ALL variants public automatically.
//    (Opposite of structs — enums are useless with private variants.)
// ============================================================
