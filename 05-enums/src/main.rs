#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


#[derive(Debug)]
struct IpAddr {
    kind : IpAddrKind,
    addr : String,
}


#[derive(Debug)]
enum IpAddrAdvanced {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
#[allow(dead_code)]
enum USState {
    Alabama,
    Alaska,
    // ...
}


#[allow(dead_code)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(USState),
}


fn main() {

    // Normal enums. Enums in Rust act somewhat like enums + unions in C/C++ (but more flexible).
    let ip1 = IpAddr{kind : IpAddrKind::V4, addr : String::from("127.0.0.1")};
    let ip2 = IpAddr{kind : IpAddrKind::V6, addr : String::from("::1")};
    println!("{:?} | {:?}", ip1, ip2);

    // Advanced enums (types are named tuples, can also be structs).
    let ip3 = IpAddrAdvanced::V4(127, 77, 134, 90);
    let ip4 = IpAddrAdvanced::V6(String::from("::1"));
    println!("{:?} | {:?}", ip3, ip4);

    // Using methods.
    ip4.say_hello();
    println!("{} {}", ip3.is_loopback(), ip4.is_loopback());

    // Special enum: `Option<T>`.
    // Rust's safer version of NULL.
    let zero_val = Some(0);
    let null_val : Option<i32> = None;  // If initializing with `None`, then must give the type hint.
    println!("{:?} {:?}", zero_val, null_val);

    // Handling `Options`. (Other useful features, see online doc.)
    println!("{}", 3 + zero_val.unwrap());
    match null_val {
        Some(num) => println!("{}", num),
        None => println!("It's NULL!"),
    }

    // `match`.
    let coin = Coin::Quarter(USState::Alaska);
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("State is {:?}", state);
            25
        },
    };
    println!("{}", value);

    // `_` placeholder.
    let number = 7;
    match number {
        1 => println!("You got 1!"),
        5 => println!("You got 5!"),
        7 => println!("You got 7!"),
        _ => (),    // `match` MUST be exhaustive.
    }

    // `if let` to handle one-armed match, when we are only interested in one arm.
    let some_val = Some(7u8);
    if let Some(i) = some_val { println!("{}", i); }
    let some_val2 : Option<u8> = None;
    if let Some(i) = some_val2 {
        println!("{}", i);
    } else {    // Can also have `else` branch, where it is equivalent to using `_` in two-armed `match`.
        println!("None!");
    }

    // Special enum: `Result<T, E>`. (Other useful features, see online doc.)
    let res : Result<i32, String> = Ok(7);
    let err : Result<i32, String> = Err(String::from("Overflow!"));
    match res {
        Ok(num) => println!("{}", num),
        Err(msg) => println!("{}", msg),
    }
    let hoping_for_a_num = err.expect("Error happened");
    println!("{}", hoping_for_a_num);
}


// Enums can have methods.
impl IpAddrAdvanced {

    fn say_hello(&self) {
        println!("Hello, world!");
    }

    // Pattern matching! One of Rust's most powerful tools.
    // `match` can not only match numerical values, but also de-packing enum types.
    fn is_loopback(&self) -> bool {
        match self {
            IpAddrAdvanced::V4(s1, s2, s3, s4) => return *s1 == 127 && *s2 == 0 && *s3 == 0 && *s4 == 1,
            IpAddrAdvanced::V6(ipstr) => return ipstr == "::1",
        }
    }
}
