use std::io::{stdin, stdout, Write};
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    // Generate secret number .
    let secret = thread_rng().gen_range(1, 101);

    // Loop indefinitely until correct guess.
    loop {
        // Read in user guess input.
        print!("Please inupt your guess: ");
        stdout().flush().unwrap();
        let mut guess = String::new();
        stdin().read_line(&mut guess)
               .expect("Failed to read your input!");

        // Convert user input to number. Ask for a retry when failed.
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number! Retrying...");
                continue;
            },
        };

        // Compare.
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Congratulations ;)");
                break;
            },
        }
    }
}
