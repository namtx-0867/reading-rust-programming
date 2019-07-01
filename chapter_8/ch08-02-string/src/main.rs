fn main() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", data);

    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("{}", s1);

    let mut s2 = String::from("lo");
    s2.push('l');
    println!("{}", s2);

    let len = String::from("Hola").len(); 
    println!("{}", len);
    
    // + 
    let s4 = String::from("hello");
    let s5 = String::from(", world!");

    let s6 = s4 + &s5;
    println!("{}", s6);
    // s4 no longer valid
    // println!("{}", s4); 
    //
    // fn add(self, a: &String) -> String {}
    // concat '+' will take ownership of s4 and append a copy of the contents of s5
    // and then return ownership of result.
    
    // concat multiple string
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");

    let d = a + "-" + &b + "-" + &c;
    println!("{}", d);

    // format! macro
    let aa = String::from("tic");
    let bb = String::from("tac");
    let cc = String::from("toe");

    let dd = format!("{}-{}-{}", aa, bb, cc);
    println!("{}", dd);
    // format! doesn't take the ownership
    // aa is still valid here!
    println!("{}", aa);

    // INDEXING
    //
    // indexing a string
    // String cannot be indexed by integer
    // let s1 = String::from("hello world");
    // let h = s1[0]; -> invalid 
    
    // String is wrapper over a Vec<u8>
    let len = String::from("hello").len();
    println!("{}", len);
    // 5 bytes long


    // Slice 
    let hello = "Здравствуйте";
    let sliced_hello = &hello[0..4];    
    println!("{}", sliced_hello);

    // Iterating
    for c in hello.chars() {
        println!("{}", c);
    }

    // Bytes
    for b in hello.bytes() {
        println!("{}", b);
    }
}
