use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // introduce game
    println!("Number Guessing Game");

    // generate random number
    let random_number = rand::thread_rng().gen_range(1..101);

    loop {
        // ask user for input
        println!("Guess a number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert user input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };

        // tell the user if the guess was right
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
