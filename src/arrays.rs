//Arrays: fixed list where elements are the same data type

//Importing memory from standard library 
use std::mem;

pub fn run() {

    //when declaring an array the data type and the length must be defined
    //and separated by a semicolon
    //the following array is immutable
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    //get single value
    println!("Single Value: {}", numbers[0]);

    //if you want to make an array mutable you can do so
    let mut prices: [f32; 4]  = [2.50, 3.20, 2.85, 1.60];

    prices[2] = 3.00;

    println!("New price: {}", prices[2]);

    //Get array length
    println!("Array length: {}", prices.len());

    //Arrays are stack allocated
    //Using size_of_val from mem library and referencing the array with &
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    //Get a slice of an array as another array
    //the index reference begins at and up to, but not including the end index
    let slice: &[f32] = &prices[1..3];
    println!("{:?}", slice);
}