use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    // + recoverable
    // + unrecoverable
    // Rust doesn't have exceptions
    // + Result<T, E>
    //
    // + panic! 
    //      + when panic occurs, the program starts unwinding
    //      + --release flag
    //

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    
    // unwrap
    // if Result is Ok, unwrap will return the value inside Ok
    // if Result is the Err variant, unwrap will call panic! marco
    //
    let another_file = File::open("hello_world.txt").unwrap();

    // expect
    // similar to unwrap
    //
    let another_file = File::open("hello_world.txt").expect("Failed to open file hello.txt");

    // Propagating Errors
    //
    match read_username_from_file() {
       Ok(s) => {
           println!("{}", s);
       },
       Err(e) => {
           panic!("{}", e);
       }
    };

    match short_read_username_from_file() {
        Ok(s) => {
            println!("{}", s);
        },
        Err(e) => {
            panic!("{}", e);
        },
    }

    match shorter_read_username_from_file() {
        Ok(s) => {
            println!("{}", s);
        },
        Err(e) => {
            panic!("{}", e);
        },
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file, 
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
