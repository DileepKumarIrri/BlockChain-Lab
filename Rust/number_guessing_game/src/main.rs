use rand::Rng;
use std::io;

fn main() {
    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Infinite loop to keep the game running until the correct guess
    loop {
        println!("Guess the number (between 1 and 100):");

        // Create a mutable String to store the user's guess
        let mut guess = String::new();

        // Read the user input from the standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess string to a number (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Compare the guessed number with the secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed it! The secret number was: {}", secret_number);
                break; // Exit the loop if the guess is correct
            }
        }
    }
}
