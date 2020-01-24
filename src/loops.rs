pub fn run() {
    let mut count = 0;

    // Infinite
    // loop {
    //     count += 1;
    //     println!("number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizbuzz")
        } else if count % 5 == 0 {
            println!("buzz")
        } else if count % 5 == 0 {
            println!("fiz")
        }

        count += 1;
    };

    // Range
    for x in 0..10 {
        println!("hi from: {}", x)
    }
}