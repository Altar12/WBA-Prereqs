use std::mem;

pub fn run() {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign element
    nums[2] = 20;
    println!("{nums:?}");

    // get single element
    println!("First element: {}", nums[0]);

    // get array length
    println!("Array length: {}", nums.len());

    // bytes occupied on stack
    println!("Array occupies {} bytes", mem::size_of_val(&nums));

    // get slice
    let slice: &[i32] = &nums[0..2];
    println!("{slice:?}");
}
