use std::env;

pub fn run() {

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Vern";
    let town = "Wairoa";

    //println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, where are you from?", name);
    } else if command == "town" {
        println!("I'm from {} bro!", town);
    } else {
        println!("That is not a valid command.");
    }
}