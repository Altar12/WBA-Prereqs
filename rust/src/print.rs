pub fn run() {
    // printing to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("{} loves building on {}", "Alice", "solana");

    // positional arguments
    println!(
        "{0} is learning {1} and {0} plans to build on {2}",
        "Alice", "rust", "solana"
    );

    // named arguments
    println!(
        "{name} like to develop on {blockchain}",
        name = "Alice",
        blockchain = "solana"
    );

    // placeholder traits
    println!("Binary {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
