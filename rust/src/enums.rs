enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let move1 = Movement::Up;
    let move2 = Movement::Down;
    let move3 = Movement::Left;
    let move4 = Movement::Right;

    move_avatar(move1);
    move_avatar(move2);
    move_avatar(move3);
    move_avatar(move4);
}
