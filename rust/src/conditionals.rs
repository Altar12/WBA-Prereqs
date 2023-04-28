pub fn run() {
    let age = 18;
    let id_checked = true;
    let knows_person = true;

    if age >= 21 && id_checked || knows_person {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && id_checked {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
