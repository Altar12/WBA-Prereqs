pub fn run() {
    let mut my_str = String::from("Hello ");

    // get length
    println!("Length: {}", my_str.len());

    // push char
    my_str.push('W');

    // push string
    my_str.push_str("orld!");

    // capacity in bytes
    println!("Capacity: {}", my_str.capacity());

    // check if empty
    println!("Empty: {}", my_str.is_empty());

    // check for substring
    println!("Contains 'World': {}", my_str.contains("World"));

    // replace
    println!("{}", my_str.replace("World", "There"));

    // loop through words
    for word in my_str.split_whitespace() {
        println!("{word}");
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{s}");

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{my_str}");
}
