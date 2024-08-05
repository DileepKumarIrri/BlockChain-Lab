
fn main() {
    //Arrays
    println!("Arrays:");
    // Declare and initialize an array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];


    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    println!("All elements in the array:");
    for number in numbers.iter() {
        println!("{}", number);
    }

    println!("Array length: {}", numbers.len());


    //tuples
    println!("\nTuples:");
    let person: (i32, f64, char) = (30, 5.8, 'A');

    println!("Age: {}", person.0);
    println!("Height: {}", person.1);
    println!("Initial: {}", person.2);

    let mixed: (&str, i32, bool) = ("Alice", 25, true);
    println!("Name: {}", mixed.0);
    println!("Age: {}", mixed.1);
    println!("Is student: {}", mixed.2);
    
}
