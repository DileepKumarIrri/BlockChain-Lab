fn main() {
    // For loop
    for i in 0..5 {
        println!("Value of i: {}", i);
        if i == 3 {
            break;
        }   
    }

    // While loop
    let mut j: i32 = 0;
    while j < 6 {
        println!("Value of j: {}", j);
        j = j + 1;
    }
}
