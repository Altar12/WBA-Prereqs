pub fn run() {
    // default i32
    let x = 200;

    // default f64
    let y = 3.14;

    // explicit type annotation
    let z: u32 = 4321;

    // extreme values of a type
    println!("Max i32: {}, max i64: {}", std::i32::MAX, std::i64::MAX);

    // boolean
    let is_active = true;

    // boolean from expression
    let is_greater = 10 < 5;

    // char
    let a = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a, face));
}
