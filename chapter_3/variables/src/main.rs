fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 10;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let z: i8 = 12;
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the value of arr is: {:?}", arr);

    let another_arr = [3; 5];
    println!("the value of arr is: {:?}", another_arr);
}
