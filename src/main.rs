#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, breadth) => length * breadth,
    }
}

fn main() {
    let circle = Shape::Circle(3.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(2.0, 3.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}
