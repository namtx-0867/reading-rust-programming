fn main() {
    // if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // loop 
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // for 
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }
}
