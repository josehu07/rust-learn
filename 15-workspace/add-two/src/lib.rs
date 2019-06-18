use rand::prelude::*;


pub fn add_two(x : i32) -> i32 {
    x + thread_rng().gen_range(1, 101) + thread_rng().gen_range(1, 101)
}
