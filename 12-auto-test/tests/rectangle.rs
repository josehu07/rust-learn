//
// Each file under `tests/` folder will be compiled as an individual test crate.
//   These "integration tests" can only touch the external APIs from your library, like users.
//

use auto_test::*;

mod common;

#[test]
fn area() {
    common::setup();
    let r = shapes::Rectangle { width : 7, height : 3 };
    assert_eq!(r.area(), 21);
}
