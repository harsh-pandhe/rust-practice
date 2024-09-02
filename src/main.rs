use std::fs;

enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    let res = fs::read_to_string("src/main.rs");

        match res {
            Ok(content) => {
                println!("{}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
