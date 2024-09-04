// A) Assigning value of one variable to another variable
fn main() {
    // Assigning value to a variable
    let x = 10;
    // Assigning value of one variable to another
    let y = x;

    println!("The value of x is: {}", x);
    println!("The value of y (copied from x) is: {}", y);

    // B) Passing value to a function
    let result = square(y);
    println!("The square of y is: {}", result);

    // C) Returning value from a function
    let result_from_function = add_ten(result);
    println!("Result after adding 10 is: {}", result_from_function);
}

// Function to demonstrate passing a value to a function and returning a value
fn square(num: i32) -> i32 {
    num * num
}

// Another function to demonstrate returning a value from a function
fn add_ten(num: i32) -> i32 {
    num + 10
}
