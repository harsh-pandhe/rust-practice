fn main() {
    let sentence: String = String::from("the quick brown fox jumps over the lazy dog");
    println!("{}", sentence);

    let first_word: String = get_first_word(sentence);
    let a: i32 = 1000;
    for _i in 0..a {
        println!("Harsh was here!");
    }
    println!("First Word is: {}", first_word);
    let sum: i32 = do_sum(10, 20);
    print!("Sum is: {}", sum);
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn do_sum(i: i32, j: i32) -> i32 {
    return i + j;
}
