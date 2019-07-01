fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}", s1, len);
    change(&s1);

    // Mutable References
    let mut s2 = String::from("hello");
    mutable_change(&mut s2);
    println!("{}", s2);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_change(some_string: &mut String) {
    some_string.push_str(", world!")
}
