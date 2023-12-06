use rand::Rng;
use std::cmp::Ordering;
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

    // convert user input to number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // tell the user if the guess was right
    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
