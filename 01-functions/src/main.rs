fn main() {
    say_hello("Jose");
    
    println!("{}", verbose_add_one(19));

    let (new_s, l) = get_a_tuple();
    println!("{} {}", new_s, l);
}


// Void function.
fn say_hello(name : &str) {
    println!("Hello world from {}!", name);
}


// Has return value.
fn verbose_add_one(x : i32) -> i32 {
    let y = {   // A let is a statement (no return value).
        let xx = x;
        xx + 1  // This is an expression (has return value).
    };          // A {} block is also an expression, returning value of the last line.

    y
    // Or `return y`, but no need.
}


// Return a tuple.
fn get_a_tuple() -> (String, usize) {
    let s = String::from("Secret!");
    let l : usize = s.len();
    (s, l)
}


// NOTICE: whenever an expression really has no return value, it returns a unit tuple `()`.
//   e.g.:
//     - Functions that do not have return value.
//     - A block expression that the last line is a statement.
//     - ...
