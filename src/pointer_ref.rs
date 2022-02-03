//Reference Pointers: point to a reference in memory

pub fn run() {

    //Primitive array
    let arr1: [u8;3] = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    //with non-primitive data types, if you assign another variable to 
    //piece of data, the first variable will no longer hold that value.
    //You'll need to use a reference (&) to point to the resource

    //Vector
    let vec1: Vec<u8> = vec![4,5,6];
    let vec2 = &vec1;     //ampersand must be used reference vec1 otherwise it cannot be copied into vec2

    println!("Values: {:?}", (&vec1, vec2));
}