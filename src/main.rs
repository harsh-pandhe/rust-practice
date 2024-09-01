fn main() {
    let s1 = String::from("hello");
    takes_ownership(&s1); // s1 is borrowed
    println!("{}", s1); // s1 is borrowed, so it can be used here

    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;

    println!("{}, {}, {}", s2, s3, s4);
    println!("{}", s1); // s1 is borrowed, so it can be used here
}

fn takes_ownership(s: &String) {
    // s is a reference to a String
    println!("{}", s);
}
