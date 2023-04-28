use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign element
    nums[2] = 20;

    // add to the vector
    nums.push(6);
    nums.push(7);

    // pop from the vector
    nums.pop();

    println!("{nums:?}");

    // get single element
    println!("First element: {}", nums[0]);

    // get array length
    println!("Vector length: {}", nums.len());

    // bytes occupied on stack
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));

    // get slice
    let slice: &[i32] = &nums[0..2];
    println!("{slice:?}");

    // loop through the vector
    for num in nums.iter() {
        println!("Number: {num}");
    }

    // iterate and mutate values
    for num in nums.iter_mut() {
        *num *= 2;
    }

    println!("Numbers: {nums:?}");
}
