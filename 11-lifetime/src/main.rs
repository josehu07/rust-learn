use std::fmt::Display;


fn main() {
    
    //
    // Lifetime concept:
    //
    // Won't compile:
    // {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    //                           // ---------+
    // }
    //
    // No problem:
    // {
    //     let x = 5;            // ----------+-- 'b
    //                           //           |
    //     let r = &x;           // --+-- 'a  |
    //                           //   |       |
    //     println!("r: {}", r); //   |       |
    //                           // --+       |
    // }                         // ----------+
    //

    // Lifetime paramter needed at functions returning a reference.
    let s1 = "abcd".to_string();
    let s2 = "xyz".to_string();
    let res = longer_string(s1.as_str(), s2.as_str());
    println!("{}", res);
    //
    // The following won't compile: `string2` does not live long enough.
    //
    // let string1 = "long string is long".to_string();
    // let result;
    // {
    //     let string2 = "xyz".to_string();
    //     result = longer_string(string1.as_str(), string2.as_str());
    // }
    // println!("The longer string is {}", result);
    //

    // Lifetime paramter needed at structs with a member who is a reference.
    let input = "Annie, Bob, Caroline".to_string();
    let card = IDCard { name : input.split(',').next().expect("No splitter found.") };
    println!("{:?}", card);
    card.welcome_and_return_name("Welcome!");

    // The `'static` lifetime lives for the entire program.
    let _s : &'static str = "HAHAHA";   // String literal is an example.

    // Combined example.
    longer_string_with_welcome_msg(s1.as_str(), s2.as_str(), "HAHAHAHA");
}


// Give explicit generic name `'a` to variables' lifetime, indicating they must all live as long as
//   a certain lifetime `'a`. Otherwise it won't compile. Concrete length of `'a` will be inferred
//   at where this function is called, and will be the intersection of all "'a-annotated" arguments'
//   lifetimes. Then, the return value's lifetime will be checked, to see whether it is within `'a`.
fn longer_string<'a>(s1 : &'a str, s2 : &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


// Give explicit generic name `'a` to members' lifetime, indicating that any instance of this struct
//   must not outlive the thing that its "'a-annotated" member points to.
#[derive(Debug)]
struct IDCard<'a> {
    name : &'a str,
    // `name : &str` won't compile.
}

impl<'a> IDCard<'a> {

    fn welcome_and_return_name(&self, welcome_msg : &str) -> &str {     // Auto ellision example.
        println!("{}", welcome_msg);
        self.name
    }
}


// Example of using generic type & lifetime annotation & traits together.
fn longer_string_with_welcome_msg<'a, T>(s1 : &'a str, s2 : &'a str, msg : T) -> &'a str
    where T : Display
{
    println!("Hello! {}!", msg);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


// NOTICE: Previously Rust requires programmers to input explicit lifetime parameters for all function with input
//   references, and for all structs with referencer members. However, Rust compiler has evolved so that if going
//   through the three procedures (see https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)
//   can give all referencers a lifetime, then it will not require an explicit lifetime annotation.
//   This is called "Lifetime Ellision", and may confuse newbees a bit.
