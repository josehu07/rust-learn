use std::collections::HashMap;


fn main() {

    //
    // Vectors.
    //

    // Creation.
    let v1 : Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];     // Using macro with initial values, then no need of type annotation.
    
    // Mut.
    let mut v3 = Vec::new();    // Similar when having `push` in the same scope.
    let mut v4 = vec![];
    v3.push(1);
    v4.push(2);
    println!("{:?} {:?} {:?} {:?}", v1, v2, v3, v4);

    // Reference the elements.
    let e1 : &i32 = &v2[2];
    let e2 : &i32 = v2.get(2).unwrap();
    // `let e3 = &v3[100];` will panic at runtime.
    let e4 = v2.get(100);
    println!("{:?} {:?} {:?}", e1, e2, e4);

    // Drop.
    let e5;
    {
        let v5 = vec![7, 8, 9];
        e5 = v5.get(1).unwrap();
        println!("{}", e5);
    }
    // `println!("{}", e5);` here does not compile: when `v5` drops, all its contents also drop,
    //                                                so reference to the second value is not valid anymore.

    // Ownership.
    #[allow(unused_mut)]
    let mut v6 = vec![4, 5, 6];
    let third = &v6[2];
    // `v6.push(7);` here does not compile: `third` immutably borrows `v6` and `push` tries to mutably borrow it.
    println!("{:?} {:?}", v6, third);

    // Combine union-like enum with vector to allow different types.
    #[derive(Debug)]
    enum Cell { Int(i32), Text(String), Float(f64) }
    let sheet = vec![
        Cell::Int(8),
        Cell::Text("hello".to_string()),
        Cell::Float(3.14)
    ];
    println!("{:?}", sheet);


    //
    // UTF-8 endoded strings.
    //   `String` is a wrapper over `Vec<u8>`.
    //

    // Creation.
    let mut s1 = String::new();
    let mut s2 = "hello".to_string();
    let mut s3 = String::from("world");
    println!("{} {} {}", s1, s2, s3);

    // UTF-8 encoded.
    let s4 = "hello".to_string();
    let s5 = "你好".to_string();
    println!("{}-{}; {}-{}", s4, s4.len(), s5, s5.len());
    // `let ch = &s4[0];` does not compile: Strings are UTF-8 encoded, so does not allow indexing.

    // Growing.
    s1.push('a');
    s2 += "xyz";
    s3.push_str("jkl");
    let s6 = format!("slot1 = {}, slot2 = {y}", s1, y = 10);
    println!("{} {} {}", s1, s2, s6);
    let s7 = s2 + &s3;  // Here `&s3` coerces into `&s3[..]`.
    // `println!("{}", s2);` here does not compile: `s2` has been moved.
    println!("{}", s7);

    // String slices.
    let hello = "Здравствуйте";
    let slice = &hello[0..4];
    // `let s = &hello[0..1];` will panic at runtime: 0~1 is not a valid UTF-8 char boundary here.
    println!("{}", slice);

    // Iterators over strings.
    for b in "नमस्ते".chars() {
        print!("{} ", b);
    }
    println!("");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("");


    //
    // Hash Maps (& Sets, but omitted here).
    //

    // Creation.
    let mut scores1 = HashMap::new();
    scores1.insert("Jack".to_string(), 87);
    scores1.insert("Jack".to_string(), 84);
    let v1 = vec![String::from("Alice"), "Bob".to_string()];
    let v2 = vec![98, 66];
    let scores2 : HashMap<_, _> = v1.iter().zip(v2.iter()).collect();
    println!("{:?} {:?}", scores1, scores2);

    // Get value.
    let name1 = "Alice".to_string();
    let name2 = "Emily".to_string();
    println!("{:?} {:?}", scores2.get(&name1), scores2.get(&name2));

    // Unordered iteration.
    for (key, val) in &scores2 {
        println!("{}: {}", key, val);
    }

    // Updateing.
    let text = "C is good and Rust is great!";
    let mut wc_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = wc_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{}", text);
    println!("{:?}", wc_map);
}


// NOTICE: There are tons of powerful standard library methods for theses collection types.
//         Always refer to the documentation when not sure how to easily implement a functionality.
