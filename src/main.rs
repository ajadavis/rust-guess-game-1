use std::io;
use std::io::prelude::*;
extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    // Do some stuff
    println!("Welcome to the guessing game!");
    pause();
    let mut number = String::new();
    println!("Enter any number to guess!");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    io::stdout().flush().unwrap();
    let trimmed = number.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            let mut rng = thread_rng();
            // let y: f64 = rng.gen_range(-10.0, 10.0);
            // println!("Number from -10. to 10.: {}", y);
            let answer = rng.gen_range(0, 10);
            if i == answer {
                // executed if condition1 true
                print!("The number you guessed is {}", i);
                println!("The correct answer is: {}", answer);
                print!("You win!");
            } else {
                // executed if both condition1 and condition2 are false
                println!("The number you guessed is {}", i);
                println!("The correct answer is: {}", answer);
                print!("Sorry, Try Again!");
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
