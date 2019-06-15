#[derive(Debug)]    // Automaticly derives debug printing.
struct User {
    name: String,
    active: bool,
}


#[derive(Debug)]
struct Point(i32, i32);     // Tuple struct (i.e. tuple with a type name).


fn main() {

    // Immutable struct.
    let user1 = new_user(String::from("Jose"));
    println!("{} {}", user1.name, user1.active);

    // Mutable struct.
    let mut user2 = new_user(String::from("Annie"));
    user2.active = false;
    println!("{:?}", user2);

    // Struct update syntax.
    let user3 = User {
        name : String::from("Lily"),
        ..user2
    };
    println!("{:#?}", user3);

    // Using tuple structs.
    let mut p = Point(5, 3);
    p.1 = 4;
    println!("{:?}", p);

    // Associated functions and methods.
    p.step_away();
    println!("{}", p.distance(&Point::center()));   // Associated funcs are namespaced by the type name.
}


fn new_user(name : String) -> User {
    User {
        name,   // When variable has the same name as the field, can use such shorthand.
        active : true,
    }
}


impl Point {

    // Associated functions (declared inside impl block, but does not take self as a parameter.)
    //   Often used as constructors, e.g. `String::from("...")`.
    fn center() -> Point {
        Point(0, 0)
    }

    // Methods.
    fn step_away(&mut self) {
        self.0 += 1;
        self.1 += 1;
    }

    fn distance(&self, other : &Point) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt()
    }
}
