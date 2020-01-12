/*
    Premetives
    ints, floats, boolean, chars, tuples, array.

    Rust is statically typed lang.
*/

use typename::TypeName;

pub fn run() {
    // default is i32
    let x = 2147483647;
    println!("type  {} is {}", x, x.type_name_of());

    // default is f64
    let y = 2.5;
    println!("type  {} is {}", y, y.type_name_of());

    // explicit type
    let z: i64 = 474747474747;
    println!("type  {} is {}", z, z.type_name_of());

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i32::MAX);

    // bool
    let is_active: bool = true;
    println!("type  {} is {}", is_active, is_active.type_name_of());

    // bool from exp
    let is_passed: bool = 10 < 5;

    // chars
    let a1 = 'm';
    println!("type  {} is {}", a1, a1.type_name_of());

    let emoji = '\u{1F600}';
    println!("type  {} is {}", emoji, emoji.type_name_of());

    println!("{:?}", (x, y, z, is_active, is_passed, a1, emoji));
}