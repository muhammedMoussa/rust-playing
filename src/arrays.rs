// array, fixed list and all elements in same data type..
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign
    numbers[2] = 22;

    // all array
    println!("{:?}", numbers);

    // single i
    println!("first: {}, last: {}", numbers[0], numbers[numbers.len() - 1]);

    // stack
    println!("array {} byets", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[0..3];
    println!("Sliced: {:?}", slice);
}