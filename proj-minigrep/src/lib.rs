//
// `lib.rs` is responsible for the main logic.
//

use std::env;
use std::error::Error;
use std::fs;


// Config parsing.
pub struct Config<'a> {
    query : &'a str,
    filename : &'a str,
    sensitive : bool,
}

impl<'a> Config<'a> {

    pub fn new(args : &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Wrong number of arguments.")
        }

        let query = &args[1];
        let filename = &args[2];
        let sensitive = env::var("CASE_INSENSITIVE").is_err();  // Check for env variable.

        Ok(Config { query, filename, sensitive })
    }
}


// Main logic.
pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.sensitive {
        search_sensitive(config.query, contents.as_str())
    } else {
        search_insensitive(config.query, contents.as_str())
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_sensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let mut results : Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results : Vec<&str> = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
