//Vectors: resizeable arrays
// you will use them more than arrays

//Importing memory from standard library 
use std::mem;

pub fn run() {

    //when declaring a vector the data type is defined in angle brackets
    //the following vector is immutable
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //add onto a vector - push adds to end
    numbers.push(6);
    numbers.push(7);

    //pop off the last value using pop
    numbers.pop();

    println!("{:?}", numbers);

    //loop through a vector using .iter()
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate through a vector using iter_mut()
    //multiply each element by 2
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Mutated vector: {:?}", numbers);

    //get single value
    println!("Single Value: {}", numbers[0]);

    let mut prices: Vec<f32>  = vec![2.50, 3.20, 2.85, 1.60];

    prices[2] = 3.00;

    println!("New price: {}", prices[2]);

    //Get vector length
    println!("Vector length: {}", prices.len());

    //Vectors are stack allocated
    //Using size_of_val from mem library and referencing the array with &
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get a slice of an array as another array
    //the index reference begins at and up to, but not including the end index
    let slice: &[f32] = &prices[1..3];
    println!("{:?}", slice);
}