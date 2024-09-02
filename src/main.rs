struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10.0 };
    let float = Point { x: 1.0, y: 4.0 };
    let string = Point { x: "Hello", y: "World" };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
    println!("string.x = {}, string.y = {}", string.x, string.y);
}
