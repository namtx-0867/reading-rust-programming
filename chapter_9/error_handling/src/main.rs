use std::fs::File;
use std::io::ErrorKind;
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
    let username_file = File::open(
}












