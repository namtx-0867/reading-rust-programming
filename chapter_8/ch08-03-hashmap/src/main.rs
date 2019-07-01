use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // two separate vector -> HashMap
    //

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", team_scores);

    // ownership
    //
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{:?}", map);

    // field_value and field_name are invalid here!
    // println!("{}", field_value);
    //


    // Accessing values
    //
    let mut languages = HashMap::new();
    languages.insert(String::from("English"), 10);
    languages.insert(String::from("Vietnamese"), 15);
    let english = String::from("English");
    let score = languages.get(&english);

    if let Some(i) = score {
        println!("{}", i);
    }

    // for
    //

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // Insert if key has no value
    //
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blye")).or_insert(50);

    // Entry is defined to return a mutabhle references to the value for the corresponding Entry
    // key if that key is exists
    //
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Updating a value based on the old value
    //
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Hasing function
    // cryptographically strong hashing function
    // implements the BuildHasher trait
}
