use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 3, 5, 3, 6, 2];
    println!("Mean of v is: {}", mean(&v));
    println!("Median of v is: {}", median(&mut v));
    println!("Mode of v is: {}", mode(&v));
    assert_eq!("apple-hay", pig_latin("apple"));
    assert_eq!("irst-fay", pig_latin("first"));
}

// Given a list of integer, use a vector and return the mean (the average value)
//
fn mean(v: &Vec<i32>) -> f32 {
    let sum: i32 = v.iter().sum();
    sum as f32 / v.len() as f32
}

// Median, middle element when sort vector 
//
fn median(v: &mut Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    v.sort();
    v[mid]
}

// Mode, the value that occurs most often
//
fn mode(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in v.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1
    }

    let mut m: i32 = v[0]; 
    let mut max_occurs: i32 = 0;

    for (key, value) in map {
        if value > max_occurs {
            m = key;
            max_occurs = value;
        }
    }
    m
}

// Pig-latin
// first -> irst-fay
// apple -> apple-hay
//
fn pig_latin(s: &str) -> String {
    let vowel = "aeiouw";
    let len = s.len();
    let first_char = s.chars().next().unwrap();

    if vowel.contains(first_char) {
        return format!("{}-hay", s);
    }
    format!("{}-{}ay", &s[1..len], first_char)
}
