use std::io;

fn main() {
    println!("Enter the size of the array:");
    
    // Read the size of the array
    let mut size_str = String::new();
    io::stdin().read_line(&mut size_str).expect("Failed to read line");
    let size: usize = size_str.trim().parse().expect("Invalid input");
    
    // Initialize a vector with the given size
    let mut arr = Vec::with_capacity(size);
    
    // Read the elements of the array
    println!("Enter the array:");

    for _ in 0..size {
        let mut element_str = String::new();
        io::stdin().read_line(&mut element_str).expect("Failed to read line");
        let element: i8 = element_str.trim().parse().expect("Invalid input");
        arr.push(element);
    }

    // Check if we can create a slice from the array
    if arr.len() > 3 {
        // Creating a mutable slice from the vector
        let slice = &mut arr[1..4];
        
        // Printing the slice
        println!("Slice: {:?}", slice);
        
        // Modifying the original vector through the slice
        slice[0] = 10;
        slice[2] = 20;
        
        // Printing the modified vector
        println!("Modified Array: {:?}", arr);
    } else {
        println!("Array does not have enough elements to create the slice.");
    }
}

