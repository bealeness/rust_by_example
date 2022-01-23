pub fn run() {
    //print to console
    println!("Hello from the print.rs file");
}

pub fn placeholders() {

    //single placeholder
    println!("Number {}", 1);

    //multiple placeholders
    println!("{}, {} and {}", "Peter", "Paul", "Mary");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Rangi", "Tokoroa", "code");

    //named arguments
    println!("{name} likes to play {game}.", name = "Barry", game = "Tennis");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, false, "G'day"));

    //basic math
    println!("579 + 432 = {}", 579 + 432);

}