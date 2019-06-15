use std::fmt::*;


#[derive(Debug)]    // Use `derive` attribute to automatically implement some traits for a type.
struct MyTypeA {
    x : i32
}


#[derive(Debug, Clone)]
struct MyTypeB {
    x : f64
}


#[derive(Debug)]
struct MyGenericType<T> {
    x : T
}


fn main() {
    
    // Implement traits for types.
    let a = MyTypeA { x : 1 };
    let b = MyTypeB { x : 7.98 };
    println!("{:?}: {} | {}", a, a.type_name(), a.description());
    println!("{:?}: {} | {}", b, b.type_name(), b.description());

    // Generic functions with trait requirements.
    show_doc1(&a);
    show_doc2(&b);
    show_doc3(&a, &b);

    // Trait bound on generic impls.
    let c = MyGenericType { x : 3.14 };
    c.print_out_x();
}


// Defining a trait.
trait Document {

    fn type_name(&self) -> String;      // A trait method.
    
    fn description(&self) -> String {   // A trait method with default implementation.
        format!("Nothing yet for this {} type!", self.type_name())   // Can call another trait method, even if
                                                                //   it does not have default implementation.
    }
}


// Implement a trait for a type.
impl Document for MyTypeA {

    fn type_name(&self) -> String {
        "MyTypeA".to_string()
    }
}

impl Document for MyTypeB {

    fn type_name(&self) -> String {
        "MyTypeB".to_string()
    }

    fn description(&self) -> String {     // Can override default implementation.
        "Just a useless type...".to_string()
    }
}


// Generic functions with trait requirements.
fn show_doc1(item : &impl Document) {        // Meaning: `*item` is of a type which implements `Document` trait.
    println!("{}", item.description());
}

fn show_doc2<T : Document + std::fmt::Debug>(item : &T) {    // Syntax sugar: trait bounds.
    println!("{:?}: {}", item, item.description());
                                            // Meaning: `*item` is of type `T`, where `T` must have implemented `Document`
                                            //   and `Display` trait.
}

fn show_doc3<T, U>(item : &T, other : &U)   // Syntax sugar: `where` clause.
    where T : Document,
          U : Clone + Document
{
    let tmp = other.clone();
    println!("{}, {}", item.description(), tmp.description());
}


// Using trait bounds to partially implement methods for generic self-defined types only when the inner type
//   satisfies certain traits.
impl<T : Display> MyGenericType<T> {

    fn print_out_x(&self) {
        println!("{}", self.x);
    }
}

impl<T : Display> Document for MyGenericType<T> {

    fn type_name(&self) -> String {
        "MyGenericType<T> where T can be displayed".to_string()
    }
}


// NOTICE: Returning a type that implements a certain trait, refer to Trait Objects.
