fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error: value used here after move
    println!("{}", s2);
}
