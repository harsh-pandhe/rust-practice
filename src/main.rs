/**
fn main() {
    let my_string = String::from("Hello, world!");
    // takes_ownership(my_string); // This will not work because my_string has been moved to takes_ownership function
    // println!("{}", my_string); // This will not work because my_string has been moved to takes_ownership function
    takes_ownership(my_string.clone()); // This will work because my_string has been cloned
    println!("{}", my_string); // This will work because my_string has been cloned
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

 */

fn main() {
    let my_string = String::from("Hello, world!");
    let my_string = takes_ownership(my_string);
    println!("{}", my_string);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string; // This will return the value of some_string to the caller
}
