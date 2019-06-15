fn main() {

    // Owned string is on heap.
    {
        let mut owned_string = String::from("Hello");   // Allocate corresponding size of mem.
        owned_string.push_str(", world!");              // Can enlarge more.
        println!("{}", owned_string);
    }   // Freed when going out of scope.

    // Simple types have "copy" semantic (`Copy` trait).
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    // Heap types have "move" semantic. This removes the problem of double free.
    let s1 = String::from("test");
    let s2 = s1;
    // Adding `println!("{}", s1);` won't compile: `s1` is no longer a valid variable.
    println!("{}", s2);

    // Heap types can be manually copied by using `Clone` trait.
    let s3 = String::from("test");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);

    // Passing in function arguments are similiar as `let` binding.
    let s5 = String::from("Life is short.");
    take_ownership(s5);
    // Adding `println!("{}", s5);` won't compile.

    // Reference & Borrowing.
    let mut s6 = String::from("Hello");     // Mutable reference requires itself to be mutable first.
    take_mut_reference(&mut s6);
    println!("{}", s6);

    // Dangling reference is not allowed without lifetime specifications.
    // let ptr;
    // {
    //     let tmp = 7;
    //     ptr = &tmp;
    // }
    // println!("{}", *ptr);    These won't compile.

    // String slices.
    let s7 = String::from("Hello, world!");
    let hello_world : &str = &s7[..];
    let hello = &s7[0..5];
    println!("{} {} {}", hello_world, hello, first_word(hello_world));

    // Other slice types.
    let arr = [1, 2, 3, 4, 5];
    let piece : &[i32] = &arr[1..4];
    println!("{:?} {}", piece, piece.len());
}


// Take ownership of passed-in `String`.
fn take_ownership(s : String) {
    println!("{}", s);
}   // Memory for value of `s` is freed here.


// Take mutable reference of the string.
fn take_mut_reference(s : &mut String) {
    s.push_str(", world!");
}   // Here `s` goes out of scope, but nothing happens to `s6`.


// Using string slices.
fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}


// NOTICE: Ownership & Lifetime rules:
//   1. Every value has a variable that is its "owner".
//   2. Each value can only have one owner at one time.
//   3. Once the owner gets out of scope, the value is dropped (for heap values, freed).
//        i.e. if the value is on heap, it is deconstructed (freed) when the owner's
//        lifetime ends. => Similar concept as C++ RAII.

// NOTICE: Rust never implicitly makes deep copies. All types requiring deep copies can be done
//   by calling the `clone()` method.

// NOTICE: At every certain timepoint, a variable can be referenced by:
//   1. Either 1 / more immutable references.
//   2. Or only 1 mutable references.
