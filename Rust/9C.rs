fn main(){
    //formatting numbers
    let int: i32 = 42;
    println!("Integer: {}", int);

    //formatting Strings
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
}