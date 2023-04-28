pub fn run() {
    greeting("Hello", "Alice");

    // bind function return value to variable
    let sum = add(4, 10);
    println!("Sum: {sum}");

    // closures
    let n3 = 10;
    let add_nums = |n1, n2| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
