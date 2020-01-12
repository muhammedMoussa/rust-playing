// vars are immutable by default
pub fn run() {
    let name = "muhammed";

    let mut age = 24;
    println!("my name is {} and i am {}", name, age);

    age = 25;
    println!("my name is {} and i am {}", name, age);

    // declear constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple vars
    let (my_name, my_age) = ("muhammed", 24);
    println!("{} is {}", my_name, my_age);
}
