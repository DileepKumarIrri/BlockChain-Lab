// Import the necessary module from the rand crate
use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 100 (inclusive)
    let random_number: u32 = rng.gen_range(1..=100);

    // Print the generated random number
    println!("Generated random number: {}", random_number);
}
