use std::io;
use std::io::Read;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;


fn main() -> io::Result<()> {
    
    // Using `panic!` macro for unrecoverable errors.
    // Use `RUST_BACKTRACE=1 cargo run` to get backtracing in panic info.
    panic!("Crash here.");
    let v1 = vec![0, 1, 2];
    v1[99];     // Accessing out-of-bound index using `[]` will trigger panic.

    // Handling recoverable errors with `Result<T, E>` enum.
    let _f : Result<File, io::Error> = File::open("hello.txt");
    let _f = match _f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Unrecoverable error when creating hello.txt: {}", err),
            },
            _ => panic!("Unrecoverable error when opening hello.txt: {}", err),
        },
    };

    // Useful methods for `Result`.
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Error opening hello.txt");
    let _f = File::open("hello.txt").unwrap_or_else(|err| {
        panic!("Error opening hello.txt: {}", err);
    });

    // Error propagation. `?` operator can ONLY be used in functions that returns a `Result`.
    println!("{}", read_from_file("hello.txt")?.len());

    Ok(())
}


// Handler to read from a file, errors propagated.
fn read_from_file(file_name : &str) -> Result<String, io::Error> {

    //
    // The following is equivalent to:
    //
    //     let f = File::open(file_name);
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(err) => return Err(err),
    //     };
    //
    //     let mut s = String::new();
    //
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(err) => Err(err),
    //     }
    //

    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)

    //
    // Normally, use this standard library function:
    //
    //    fs::read_to_string(file_name)
    //
}


// NOTICE: `?` operator is a syntax sugar for the `try!` macro. `?` operator always calls `from` method in its
//             `From` trait, which converts the inner error type to the error type in the return type of the
//             current function.
// NOTICE: Always refer to the standard library document for useful methods and traits.
