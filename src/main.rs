struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("harshpandhehome@gmail.com"),
        username: String::from("harshpandhe"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1 email: {}", user1.email);
    println!("User1 username: {}", user1.username);
    println!("User1 active: {}", user1.active);
    println!("User1 sign_in_count: {}", user1.sign_in_count);
}
