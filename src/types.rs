/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits
each type integer takes in memory) i32 is most common
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//Rust is a statically typed language meaning that the compiler must know
//the type of a variable at compile time, but it is not required,
//often the compiler can infer what the data type of a variable is

pub fn run() {
    //by default x will be "i32"
    let x = 22;
    //by default y will be "f64"
    let y = 32.3;
    //add explicit type
    let z: i128 = 1234543234567876567890;
    // find maximum size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    
    //Boolean
    let is_active: bool = true;

    //get boolean from expression
    let is_greater: bool = 10 > 20;

    //char type - unicode characters
    //char can only be declared using single quotations, double quotations are for string types in rust
    let a1: char = 'h';
    let face: char = '\u{1F600}';

    println!("{:?}", ( x, y, z, is_active, is_greater, a1, face));
}