fn main() {

    // If & Else.
    let num = 3;
    let state = if num > 5 {    // Condition (called "arm") must be a bool!
        2
    } else if num < 2 {
        0       // Types cannot be mismatched. All branches should be same here.
    } else {    // becaue we are in a `let` statement.
        1
    };
    println!("{}", state);

    // Infinite loop.
    const LIMIT : u8 = 7;
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter >= LIMIT {
            break counter;  // Returning a value from a loop!
        }
    };
    println!("{}", res);

    // While loop.
    let mut i = 3;
    while i > 0 {
        print!("{} ", i);
        i -= 1;
    }
    println!("Go!!!");

    // For loop.
    let arr = ['A', 'B', 'C', 'D', 'E'];
    for symbol in arr.iter() {
        print!("{} ", symbol);
    }
    println!("Finished");
    for countdown in (1..4).rev() {     // Or use `(1..=3)`, which includes the final val.
        print!("{} ", countdown);
    }
    println!("Go!!!");
}


// NOTICE: Rust does NOT do automatic type conversion implicitly!
