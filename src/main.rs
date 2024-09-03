use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let random_number :u32 = rng.gen();
    println!("Random number: {}", random_number);
}
