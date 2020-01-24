pub fn run() {
    greeting("Hi", "Muhammed");

    let get_sum = add(70, 4);
    println!("sum: {}", get_sum);

    let year = 2020;
    let get_age = |year_of_birth: i32| year - year_of_birth;
    println!("you are: {}", get_age(1995));

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("sum: {}", add_nums(5, 4))


}

fn greeting(greet: &str, greeted: &str) {
    println!("{} {}, welcom!", greet, greeted);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}