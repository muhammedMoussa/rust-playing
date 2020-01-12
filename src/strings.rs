// premitive str == immutable fixed length in memory.
// String = grwoable/

use typename::TypeName;

pub fn run() {
    let mut hello = String::from("Hello World!");
    println!("type  {} is {}", hello, hello.type_name_of());

    // length
    println!("Length: {}", hello.len());

    // capacity in bytes
    println!("capacity: {}", hello.capacity());

    // push
    println!("{:?}", hello.push_str("!"));

    // empty
    println!("contains: {}", hello.contains("Hello"));

    // replace
    println!("replace: {}", hello.replace("World", "There"));

    // iteriate
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}