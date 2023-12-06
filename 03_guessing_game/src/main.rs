use rand::Rng;
use std::io;

fn main() {
    // introduce game
    println!("Number Guessing Game");

    let random_number = rand::thread_rng().gen_range(1..101);

    // prompt user for guess
    let mut guess = String::new();
    println!("Guess a number:");

    // read user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // clean up user input
    guess = guess.trim().to_string();

    // tell user what they guessed
    println!("You guessed {guess}, the answer was {random_number}");
}
