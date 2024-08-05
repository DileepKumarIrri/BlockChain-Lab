use std::io;


fn main() {
    let mut input = String::new();

    println!("Enter 1st number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number1: i32 = input.trim().parse().expect("Invalid number");
    let mut input = String::new();

    println!("Enter 2nd number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number2: i32 = input.trim().parse().expect("Invalid number");

    let number = number1 + number2; 

    println!("Result: {}", number);
}