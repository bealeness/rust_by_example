// primitive str: Immutable, fixed-length string somewhere in memory
//String: Growable, heap-allocated data structure - Use when you need to midify or own string data

pub fn run() {
    
    let good_bye = "Good Bye"; //primitive str
    let mut hello = String::from("Hello "); //String - note you still need to mut it

    //push a char to a String use
    hello.push('W');

    //push a string to a String use
    hello.push_str("orld");

    println!("{} and {}", hello, good_bye);

    //Get length
    println!("hello length: {}", hello.len());
    println!("good_bye length: {}", good_bye.len());

    //Get memory capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if string variable is empty?
    println!("Empty: {}", hello.is_empty());

    //Check if a string contains a sub string
    println!("Contains 'World': {}", hello.contains("World"));

    //Replace a sub string with another
    println!("Replace World with There: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with exact byte capacity
    let mut s = String::with_capacity(10);
    s.push('B');
    s.push('S');

    //Assertion testing - will only print something if the test fails
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    //check docs for more string modules

}