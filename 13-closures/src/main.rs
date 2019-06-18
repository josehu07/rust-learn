use std::thread;
use std::time::Duration;


fn main() {
    
    // Simple closure: A thing (anonymous function) for storing a piece of code.
    let expensive_add = |num1, num2| {
    // Can annotate types: `let expensive_add = |num1 : i32, num2 : i32| -> i32 {`.
        thread::sleep(Duration::from_secs(2));
        num1 + num2
    };
    println!("{}", expensive_add(2, 3));    // Compiler will infer the closure signature for
                                            //   us the first time it is called.

    // Closure as a struct member.
    let expensive_gen = || {
        thread::sleep(Duration::from_secs(2));
        7
    };
    let mut generator = Cacher::new(expensive_gen);
    println!("{}", generator.value());
    println!("{}", generator.value());

    // Closures can access the variables in its environment: the scope it is defined in.
    let x1 = 1;
    let mut x2 = 2;
    let x3 = vec![3, 3, 3];
    let equal_to_x = |n| n == x1;       // Has traits: `FnOnce`, `Fn`.
    let mut double_x = || x2 *= 2;      // Has traits: `FnOnce`, `FnMut`.
    let consume_x = move || x3;         // Has traits: `FnOnce`.
    println!("{}", equal_to_x(1));
    double_x();
    double_x();
    println!("{}", x2);
    consume_x();
    // `println!("{:?}", x3);` here won't compile: `consume_x` closure moves `x3`, and vectors do
    //   not have `Copy` trait yet.
}


// A cacher that lazily evaluates a func when called, and caches the result.
struct Cacher<T>
    where T : Fn() -> i32   // `Fn` trait to indicate the type is a function / closure.
{                           //   `FnMut` / `FnOnce` are also available.
    func : T,
    value : Option<i32>,
}

impl<T> Cacher<T>
    where T : Fn() -> i32
{

    fn new(func : T) -> Cacher<T> {
        Cacher { func, value : None }
    }

    fn value(&mut self) -> i32 {
        match self.value {
            Some(val) => val,
            None => {
                let val = (self.func)();
                self.value = Some(val);
                val
            }
        }
    }
}
