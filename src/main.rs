fn main() {
    let value = is_even(6);
    if value == true {
        println!("Even");
    } else {
        println!("Odd");
    }
}

fn is_even(n:i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    return false;
}