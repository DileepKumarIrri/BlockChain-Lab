fn main()
{
    let mut number1: i32 = 5;
    let mut number2: i32 = 6;

    println!("Numbers before swapping {0}, {1}", number1, number2);

    number1 = number1 ^ number2;
    number2 = number1 ^ number2;
    number1 = number1 ^ number2;
    println!("Numbers before swapping {0}, {1}", number1, number2);
}