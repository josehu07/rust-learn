//
// Cargo will auto-generate a test snippet when we create a library project using `--lib`.
//   Run the tests with `cargo test`. See handful options by `cargo test --help`.
//
#[cfg(test)]    // Module annotated with `cfg(test)` attribute becomes a unit test module.
mod tests {

    use super::*;

    // A function becomes a test when given the `test` attribute.
    #[test]
    fn this_one_succeeds() {
        assert_eq!(2 + 2, 4);
    }

    // This test will definitely fail.
    #[test]
    #[ignore]   // Ingore this test.
    fn this_one_explodes() {
        assert_eq!(1 + 1, 3);
    }

    //
    // Useful macros for testing's use. NOTE: Better to split tests on different functionalities into
    //   different test functions (e.g. one for area, one for larger can hold smaller, and one for
    //   smaller cannot hold larger). Here is not a good example of writing tests.
    //
    // In my opinion:
    //   1. Write separate test functions for every individual aspect of every functionality, or
    //   2. Use customized panic outputs to clearly indicate what each assertion means.
    //
    #[test]
    #[should_panic(expected = "Type convertion failed!")]   // Indicates that the test should panic.
    fn check_rectangle() {
        let r1 = shapes::Rectangle { width : 4, height : 3 };
        let r2 = shapes::Rectangle { width : 3, height : 2 };
        
        assert_eq!(r1.area(), 12);  // Requires `PartialEq` & `Debug` trait.
        assert_ne!(r2.area(), 99);  // ''
        assert!(r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
        assert!(r1.can_hold(&r2), "Rectangle sized ({}, {}) should contain another sized ({}, {})!",
                                  r1.width, r1.height, r2.width, r2.height);    // Customized output.
        
        let x : i32 = -3;
        let y : i32 = -771;

        let _r = crate::shapes::Rectangle::new_from_i32(x, y).unwrap();     // Should panic.
    }

    // Tests can return a `Result` enum as well, instead of panicking.
    #[test]
    fn check_rectangle_with_result() -> Result<(), String> {
        let x : i32 = 21;
        let y : i32 = 99;

        let _r = crate::shapes::Rectangle::new_from_i32(x, y)?;
        Ok(())
    }
}


// Example library content.
pub mod shapes {

    use std::convert::TryFrom;

    pub struct Rectangle {
        pub width : u32,
        pub height : u32,
    }

    impl Rectangle {

        pub fn new_from_i32(x : i32, y : i32) -> Result<Rectangle, String> {
            let width = u32::try_from(x);
            let height = u32::try_from(y);

            if width.is_err() || height.is_err() {
                Err("Type convertion failed!".to_string())
            } else {
                Ok(Rectangle { width : width.unwrap(), height : height.unwrap() })
            }

        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_hold(&self, other : &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }
}


// NOTICE: `cargo test [OPTIONS] -- [OPTIONS]`, before `--` are options passed to Cargo, and after
//   are options passed to the tester binary.

// NOTICE: Tests are run in parallel in default.
