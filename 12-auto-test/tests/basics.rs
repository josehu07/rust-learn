//
// Each file under `tests/` folder will be compiled as an individual test crate.
//   These "integration tests" can only touch the external APIs from your library, like users.
//

mod common;

#[test]
fn will_succeed() {
    common::setup();
    assert_eq!(2 + 2, 4);
}
