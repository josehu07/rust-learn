// Package (cargo project) -> Crates [-> Modules].

//
// `$ cargo new ...` creates a package, along with a `cargo.toml` to direct how to build the crates inside.
//   `src/main.rs` is the default binary crate `packages`'s root;
//   `src/lib.rs` is the default library crate `packages`'s root;
//   We can also make multiple binaries crates in one package (placing them in `src/bin`);
//   But we can only have 0 / 1 library crate in one package.
//
// So the package `packages` now contains three crates:
//   binary packages: `packages`, `hello`
//   library package: `packages`
//


// Using `use` keyword to bring things into scope.
use rand::Rng;
use packages;
use packages::front_of_house::hosting as welcome;   // Renaming using `as`.
// use packages::front_of_house::hosting::add_to_waitlist;  // Idomatically, we do not directly use function names.
#[allow(unused_imports)]
use std::io::{self, Write};     // Packing things under the same path.
#[allow(unused_imports)]
use std::collections::*;        // Glob operator.


fn main() {
    
    // Different crates.
    let a = rand::thread_rng().gen_range(1, 101);                   // Using the `Rng` under `rand` namespace.
    let b = packages::Rng::new(7, String::from("Hello, world!"));   // Using the `Rng` under my `packages` namespace.
    println!("{:?} {:?}", a, b.val);

    // Once brought into scope, then can omit the prefixing paths.
    welcome::add_to_waitlist();
    packages::serving::take_order();    // Can write this way because of the `pub use` in `lib.rs`.

}
