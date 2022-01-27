//Tuples group together values of different types
//Max 12 elements - doesn't have to be the same type like an array

pub fn run() {
    //you need to declare the types of each value in the tuple
    let person: (&str, &str, i8) = ("Beale", "Sydney", 38);

    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);

}