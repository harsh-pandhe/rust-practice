fn main() {
    stack_fn();
    heap_fn();
    update_fn();
}

fn stack_fn() {
    let x = 5;
    let y = x;
    println!("Stack function: x = {}, y = {}\n", x, y);
}

fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined String is {}\n", combined);
}

fn update_fn() {
    let mut s = String::from("Initial String");
    println!("Before update: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {}",
        s.capacity(),
        s.len(),
        s.as_ptr() as usize
    );

    for _ in 0..100 {
        s.push_str(" and some additional text");
        println!(
            "Capacity: {}, Length: {}, Pointer: {}",
            s.capacity(),
            s.len(),
            s.as_ptr() as usize
        );
    }
}
