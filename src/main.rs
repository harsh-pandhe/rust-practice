use std::fs;

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
