use std::fs;

enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    fs::read_to_string("src/main.rs").map(|contents| {
        println!("{}", contents);
    })
    .map_err(|err| {
        eprintln!("Error: {}", err);
    });
}
