// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language eg. if you declare a variable inside
// a function it loses scope outside of that function

pub fn run() {
    let name = "Beale";
    let mut age = 38; //mut to make the variable mutable

    println!("My name is {} and I am {}.", name, age);

    age = 39;

    println!("My name is {} and I am {}.", name, age);

    //define a constant - you do need to define the data type
    //and variable names need to be uppercase
    const ID: i32 = 0227; 
    println!("ID: {}", ID);

    //assign multiple variables at once
    let ( my_name, my_age ) = ("Beale", 38);
    println!("{} is {}.", my_name, my_age);
}