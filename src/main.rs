mod guess;

use guess::Guess;
use std::io;

fn main() {
    helper();
}

fn helper() {
    match require_guess() {
        Some(guess) => println!("You guessed: {}", guess.value()),
        None => println!("You didn't guess a number."),
    }
    helper();
}

fn require_guess() -> Option<Guess> {
    let mut input = String::new();

    println!("Please input your guess.");

    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }

    let result = input.trim().parse();
    match result {
        Ok(num) => Guess::new(num).ok(),
        Err(_) => None,
    }
}
