use std::ops::{Deref, DerefMut, Drop};
use std::rc::Rc;
use std::fmt::Display;


fn main() {
    
    //
    // `Box<T>`: Store data on heap rather than stack. Useful for:
    //
    //   1. Object with unknown size at compile time, but used within exact size restrictions.
    //   2. Transfer ownership of a large object, without copying it.
    //   3. Specify types that implement a certain trait, not caring about what they are exactly.
    //

    {
        let x = 5;
        let y = Box::new(5);
        assert_eq!(x, *y);      // A smart pointer owns data but acts like a reference.

        let one_box = MyBox(1);
        let mut two_box = MyBox(2);
        println!("{}", *one_box);    // Deref coersion: Compiler sees this as `*(my_box.deref())`.
                                     //                 Can also automatically add `*`s.
        *two_box += 1;
        println!("{}", *two_box);
    }   // `y`, `one_box`, and `two_box` will drop here (RAII principle).

    // Boxes are often used for Trait Objects: specify a dynamic type that implements a trait.
    let mut v : Vec<Box<dyn Display>> = vec![];
    v.push(Box::new(String::from("Hahaha")));
    v.push(Box::new("Sexy trait"));
    v.push(Box::new(123.789));
    println!("{} {} {}", v[0], v[1], v[2]);     // Trait objects use Dynamic Dispatch.


    //
    // `Rc<T>`: Reference counting type. See also `Weak<T>`.
    //

    {
        let x = Rc::new(1);
        {
            let _y = x.clone();
            let _z = x.clone();
            println!("{}", Rc::strong_count(&x));
        }
        println!("{}", Rc::strong_count(&x));
    }   // `x` drops here (reference count goes to zero).


    //
    // `RefCell<T>`: A box that allows runtime borrow checking and interior mutable reference into
    //   an immutable cell.
    //
    unimplemented!();
}


// Boxes are inehrently just a generic tuple struct.
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {

    fn drop(&mut self) {
        println!("Box dropped!");
    }
}


// NOTICE: Smart pointers, though called pointers, are often a data structure that "owns" a piece
//   of memory on heap, and wrapping over it by adding metadata and useful methods. `String` and
//   `Vec<T>` are both smart pointers. Smart pointers are often implemented using structs, but
//   must provide `Deref` and `Drop` trait.


// NOTICE: Trait objects can only be built upon Object-Safe Traits. that means the trait cannot
//   have generic parameters, and its methods cannot return `Self`.
