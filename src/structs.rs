//Structs: used to create custom data types
//similar to classes in python


//Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//Tuple Struct - notice no space between struct name and tuple
struct Sound(u8, u8, u8);

//Function Struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    //Construct Person
    fn new_person(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Get full name - self (referencing self, same as this in javascript)
    fn full_name(&self) -> String {
        //format is same as println but it doesn't print
        //don't add semi colon because you want to return the format line
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    } 

    //Name to tuple - note you want to return so no semi colon
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    //change a value in the struct
    c.red = 200;

    let mut s = Sound(10, 20, 30);

    //change a value in the tuple struct
    s.2 = 200;

    let mut p = Person::new_person("John", "Keefe");



    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    println!("Sound: {}, {}, {}", s.0, s.1, s.2);

    println!("{} {}", p.first_name, p.last_name);

    //Set last name
    p.set_last_name("Williams");

    //now using the full_name function
    println!("Full Name: {}", p.full_name());

    //return tuple
    //since it's a tuple you must use the debug trait in the placeholder
    println!("Person Tuple: {:?}", p.to_tuple());
}