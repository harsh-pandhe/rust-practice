fn find_first_a(s: &str) -> Result<usize, String> {
    for(index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Ok(index);
        }
    }
    return Err("No 'a' found".to_string());
}

fn main() {
    let s = "harsh";
    let res = find_first_a(s);
    match res {
        Ok(index) => println!("Found 'a' at index {}", index),
        Err(message) => println!("{}", message),
    }
}