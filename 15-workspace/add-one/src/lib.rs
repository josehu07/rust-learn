use rand::prelude::*;


pub fn add_one(x : i32) -> i32 {
    x + thread_rng().gen_range(1, 101)
}
