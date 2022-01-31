//Functions: used to store blocks of code for reuse

pub fn run() {
    greeting("Kia ora", "Rangi");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you!", greet, name);

    //bind function values to variable
    let get_sum = add(6, 7);
    println!("Sum: {}", get_sum);

    //Closure function - can use outside variable (such as n3)
    let n3:i32 = 77;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(23, 42));
}

// -> denotes the returned value
// not using a semicolon after n1 + n2 shows that you want a returned value
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}