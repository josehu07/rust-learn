#[allow(dead_code)]
enum Craps<'a> {
    MyInt(i32),
    MyFloatRef(&'a f64),
    MyStr(&'a str),
    Baba,
    Haha,
    Point { x : i32, y : i32 },
    Nil,
    Null,
}


fn main() {
    
    // In `match` arms: Refutable or Irrefutable.
    let x = Craps::MyInt(9);
    match x {
        Craps::MyInt(7) => println!("Gotcha!"),
        Craps::MyInt(8 ... 12) => println!("Gotcha!"),      // Range with `...`.
        Craps::MyInt(_) => println!("An int which is not 7..."),
        Craps::MyFloatRef(&num) => println!("{}", num),
        Craps::Baba | Craps::Haha => println!("Funny!"),    // OR with `|`.
        Craps::Point { x : _, y : 0 } => println!("On the x-axis!"),
        Craps::Point { x : 0, .. } => println!("On the y-axis!"),   // Ignore rest part.
        Craps::Point { x : x_value @ 5 ... 10, .. } => println!("{} is in range!", x_value),    // `@` op.
        _ => println!("Nothing"),
    }

    // In `if let`: Only Refutable.
    let x : Craps = Craps::MyInt(7);
    if let Craps::MyFloatRef(_) = x {}
    else if false {}
    else if let Craps::MyInt(x) = x {
        println!("{}", x);
    }

    // In `while let`: Only Refutable.
    let mut v = vec![1, 2, 3];
    while let Some(item) = v.pop() {
        print!("{} ", item);
    }
    println!("");

    // In `for` loops: Only Irrefutable.
    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("At {} is {}", idx, val);
    }

    // In `let` binding: Only Irrefutable.
    let (x, _, y, _, _, z) = (3, 4, 5, 7, 10, 2);
    println!("{} {} {}", x, y, z);

    // In function parameters: Only Irrefutable.
    let pair = (7, 12);
    println!("{}", calc_difference(&pair, 7));

    // Using Match Guards.
    let x = Some(3);
    match x {
        Some(x) if x > 5 => println!("Above"),
        Some(_) => println!("Below"),
        _ => println!("Error"),
    }
}


fn calc_difference(&(x, y) : &(i32, i32), _ : u32) -> i32 {
    x - y
}


// NOTICE: Refutable   - a pattern that can fail to match, e.g. `Some(x)` for an `Option`;
//         Irrefutable - a pattern that always succeed to match, e.g. `(x, y)` for a dual-Tuple;
