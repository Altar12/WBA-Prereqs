use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Alice";
    let status = "100%";

    if command == "hello" {
        println!("Hello {name}, how are you doing?");
    } else if command == "status" {
        println!("Status is {status}");
    } else {
        println!("Invalid command");
    }
}
