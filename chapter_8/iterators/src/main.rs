fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50; // dereference operator
    }
    println!("{:?}", v2);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Blue")),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
