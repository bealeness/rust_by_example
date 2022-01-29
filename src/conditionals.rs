//Conditionals: used to check the codition of something and act on the result
//&& operator for 'and'
//|| operator of 'or'

pub fn run() {

    let age: u8 = 23;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you are not old enough to drink.");
    } else {
        println!("Bartender: Can I see some ID please?")
    }

    //shorthand If
    let of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", of_age);
}