// src/main.rs and src/lib.rs are called "crate roots". Either files content for a module named
// crate at the root of the crate's module structure called the module tree.

#![allow(unused)]

use crate::back_of_house::Breakfast;
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
        pub fn seat_at_table() {
            println!("Seating at table");
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("Taking order");
        }
        fn serve_order() {
            println!("Serving order");
        }
        fn take_payment() {
            println!("Taking payment");
        }
    }
}

fn deliver_order() {
    println!("Order delivered");
}

mod back_of_house {
    // All an enum's variants are public if you make the enum public; unlike a struct where you
    // have to specify the public fields.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        // Only toast is public. seasona_fruit is private
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // If a function takes self, &self, or &mut self, it's a method.
        // If it does not take self, like summer below, it's an associated function, not a method.
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
        println!("Order cooked");
    }
}

// `use` only creates the shortcuts for the scope you put it in.
// I used `super` to bring `hosting` into scope. But you can also move the code below inside `mod
// customer`.
use crate::front_of_house::hosting;

// When bring a function into scope, include the function's  parent module. When bring a struct,
// enum or other items, use the full path.
// The exception is when bringing in two items with the same name into scope with use. For example:
// use std::fmt
// use std::io;
// fmt: Result
// io:  Result
// Or, we can use the `as` keyword to give the type a new local name. For example:
// use std::fmt::Result;
// use io::fmt::Result as IoResult;

// Re-exporting is when you use pub and use together. This lets code outside the scope refer to the
// name as it had been defined in that scope.

// Nested paths:
// use std::{cmp::Ordering, io};

// Instead of this:
// use std::io;
// use std::io::Write;
// Do this:
// use std::{self, Write};

// Using the Glob operator to bring all public items defined in a path into scope:
// use std::collections::*;

mod customer {
    use super::*;
    // This function is part of our crate's public API because the function is marked with `pub`
    pub fn eat_at_restaurant() {
        // We can do this because we brought the add_to_waitlist() into scope with the use keyword.
        hosting::add_to_waitlist();
        // Order a summer breakfast with Rye toast.
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // We changed our mind about the bread we want
        meal.toast = String::from("Wheat");
        println!("I'd like the {} toast please", meal.toast);
        // Absolute path. Preference is to use absolute path over relative path
        crate::front_of_house::hosting::add_to_waitlist();
        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}
