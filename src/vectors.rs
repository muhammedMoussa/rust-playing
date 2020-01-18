// vactors, not fixed list and all elements in same data type..
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Re-assign
    numbers[2] = 22;

    // push
    numbers.push(74);

    // all array
    println!("{:?}", numbers);

    // single i
    println!("first: {}, last: {}", numbers[0], numbers[numbers.len() - 1]);

    // stack
    println!("array {} byets", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[0..3];
    println!("Sliced: {:?}", slice);

    // loop
    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    // loop and mut
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("Mut Number: {:?}", numbers);

}