struct Person {
    name: String,
    age: u32,
    height: f64,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        height: 5.8,
    };


    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Height: {}", person.height);

    // Create a mutable instance
    let mut mutable_person = Person {
        name: String::from("Bob"),
        age: 25,
        height: 6.1,
    };

    // Modify fields
    mutable_person.age += 1;
    println!("Updated Age: {}", mutable_person.age);
}
