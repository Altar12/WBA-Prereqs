pub fn run() {
    let name = "Alice";
    let mut age = 25;
    println!("My name is {} and I am {}", name, age);
    age = 26;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 202;
    println!("ID: {}", ID);

    // assigning multiple vars
    let (my_name, my_age) = ("Bob", 30);
    println!("{} is {}", my_name, my_age);
}
