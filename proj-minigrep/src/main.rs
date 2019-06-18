//
// `main.rs` is responsible for:
//
//   1. Call the argument parsing logic;
//   2. Set up any other configurations;
//   3. Call `run()`;
//   3. Handle errors returned from `run()`;
//


use std::env;
use std::process;
use minigrep::*;


fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("ERROR parsing arguments: {}", err);  // Write to stderr.
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("ERROR when running: {}", err);
        process::exit(1);
    }
}
