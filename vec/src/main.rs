fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    // push 
    v2.push(4);
    v2.push(5);

    let third: &i32 = &v2[2];
    println!("third: {}", third);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let first = &v2[0];

    // error 
    // v2.push(10);
    // println!("The first element is: {}", first);
    
    // Iterating over v2
    for i in &v2 {
        println!("{}", i);
    }

    println!("v: {:?}", v);
    println!("v2: {:?}", v2);
}
