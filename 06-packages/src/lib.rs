//
// Defines a struct also called `Rng`, but namespaced under `packages` instead of `rand`.
//

#[derive(Debug)]
pub struct Rng {
    key : i32,          // Private fields in public struct are still private.
    pub val : String,
}

impl Rng {
    pub fn new(key : i32, val : String) -> Rng {
        Rng {           // For sturcts with private fields, must provide public constructors.
            key,        //   Otherwise users can never create them.
            val,
        }
    }
}


//
// Defining modules (paths). Corresponding module tree:
//
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
//
// Here, the name `crate` means this library crate itself.
//

#[allow(dead_code)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}

        pub fn ask_to_leave() {}
    }

    pub mod serving {
        pub fn take_order() {}  // Making things public.

        fn serve_order() {}     // Default: private, cannot use outside the module scope.

        fn take_payment() {
            super::hosting::ask_to_leave();     // Relative path: super means the same as `..` in Linux FS paths.
        }
    }
}

pub use front_of_house::serving;    // Make things used also public to users of this library.

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Absolute path (starting from root name `crate`).
    front_of_house::hosting::seat_at_table();           // Relative path.
    serving::take_order();
}
