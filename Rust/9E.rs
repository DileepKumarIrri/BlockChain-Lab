fn main()
{
    //Bitwise operators
    let a: i32 = 2;
    let b: i32 = 3;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("Using and operator, a & b: {}", a & b);
    println!("Using or operator, a | b: {}", a | b);
    println!("Using xor operator, a ^ b: {}", a ^ b);
    println!("Using not operator, ~a: {}", !a);
    println!("Using not operator, ~b: {}", !b);
    println!("Using left shift operator, a << 1: {}", a << 1);
    println!("Using right shift operator, a >> 1: {}", a >> 1);

    //logical operators
    let x: bool = true;
    let y: bool = false;

    println!("\nLogical Operators:");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("x && y = {}", x && y);
    println!("x || y = {}", x || y); 
    println!("!x = {}", !x); 
}