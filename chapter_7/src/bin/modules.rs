// ============================================================
// Chapter 7.2 — Defining Modules to Control Scope and Privacy
// ============================================================
// Modules group related code and control visibility (privacy).
// Everything inside a module is PRIVATE by default.
// The crate root (this file) is the implicit top-level module.
// ============================================================

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist called");
            super::serving::take_order();
        }

        fn seat_at_table() {
            println!("seat_at_table called");
        }
    }

    mod serving {
        pub fn take_order() {
            println!("take_order called");
        }

        fn serve_order() {
            println!("serve_order called");
        }

        fn take_payment() {
            println!("take_payment called");
        }
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();

    // ---- Intentional error examples (uncomment to see compiler errors) ----

    // ERROR 1: module `serving` is private — main() can't look into it
    // front_of_house::serving::take_order();
    // → error[E0603]: module `serving` is private

    // ERROR 2: function `seat_at_table` is private — even though hosting is pub
    // front_of_house::hosting::seat_at_table();
    // → error[E0603]: function `seat_at_table` is private

    // ERROR 3: removing `pub` from hosting — private module blocks the whole path
    // (change `pub mod hosting` to `mod hosting` above, then try:)
    // front_of_house::hosting::add_to_waitlist();
    // → error[E0603]: module `hosting` is private
}

// ============================================================
// Key rules from 7.2:
// 1) `mod {}` defines an inline module.
// 2) Everything inside a module is private by default.
// 3) `pub` exposes modules/items to code outside the parent module.
// 4) Sibling modules can see each other, but items inside still need `pub`.
// 5) `super::` moves up one level in the module tree.
// 6) Privacy is checked at every step of a path.
// 7) The top-level root module of this crate is `crate`.
// ============================================================
