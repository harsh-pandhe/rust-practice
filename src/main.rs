fn main() {
    let s1 = String::from("hello");
    // s1.push_str(", world!"); // error: cannot borrow `s1` as mutable, as it is not declared as mutable
    let mut s2 = String::from("hello");
    s2.push_str(", world!");

    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let mut s3 = String::from("hello");
    update_str(&mut s3);

    println!("s3: {}", s3);
}

fn update_str(s: &mut String) {
    s.push_str(", world!");
}
