// if
pub fn run() {
    let age: u8 = 22;
    let check_id = false;
    let client = true;

    // if else
    if age >= 21 && check_id || client{
        println!("Welcome!")
    } else if age < 21 && !check_id{
        println!("Where Id?")
    } else {
        println!("Go Home.")
    }

    // shorthand
    let can_enter = if age >= 21 {true} else {false};
    println!("Accepted: {}", can_enter);
}