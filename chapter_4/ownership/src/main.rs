fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("The value of y is : {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); Error because s1 was moved.

    let str = String::from("hello");
    takes_ownership(str);

    // println!("{}", str); Error becuase str was moved.
    let a = 5;
    makes_compy(a);
    println!("{}", a);

    let s3 = gives_ownership();
    println!("{}", s3);
    let s4 = String::from(", world");
    let s5 = takes_and_give_back(s4);
    // println!("{}", s4); Error because s4  was moved 
    println!("{}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_compy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}
