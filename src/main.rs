#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Up;
    println!("Direction: {:?}", direction);
}
 