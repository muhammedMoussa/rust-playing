pub fn run() {
    // simple print
    println!("hello from external file.");

    // formating
    println!("its program number: {} in {}", 1, "rust");

    // index args
    println!(
        "{0} looks {1} language, i like {1} things.",
        "Rust" , "cool"
    );

    // named args
    println!("hello {name}", name = "world");

    // math
    println!("{}", 70 + 4);
}