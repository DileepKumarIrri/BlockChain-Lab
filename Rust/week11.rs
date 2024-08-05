use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter number n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Invalid number");

    if n > 0 {
        println!("{} is positive", n);
        if n % 2 == 0 {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }
}
