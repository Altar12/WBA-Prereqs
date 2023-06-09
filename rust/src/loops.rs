pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("Number: {count}");
        if count == 20 {
            break;
        }
    }

    // while loop
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{count}");
        }

        count += 1;
    }

    // for range
    for count in 1..=100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{count}");
        }
    }
}
