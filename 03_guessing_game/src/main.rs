use std::io;

fn main() {
    // introduce game
    println!("Number Guessing Game");

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
    println!("You guessed {guess}");
}
