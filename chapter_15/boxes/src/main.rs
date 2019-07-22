// Boxes allow you to store data on Heap instead of stack
// What remains on the stack is the pointer to the heap data.
//
// boxes don't have performance overhead, other than storing thire data on the heap instead of on
// the stack
//
// Usage
//      + when you have a type whose size can't be know at compile time and you want use value of
//      that type in a context that requires an exact size
//      + have a large amount of data that you want to transfer ownership but ensure the data won't
//      be copied when you do so.
//      + when you want to own a value and you care only that it's type that implements a
//      particular trait rather than being of a specific type
//
//      Transferring ownership of a large amount of data can take a long time because the data is
//      copied around the stack.
//      -> Improve perf by store large amount of data to the heap in a box.
//      -> only small amount of pointer data is copied around on the stack.
//
// Enable Recursive Type
//      + at compile time, Rust need to know how much space a type takes up.
//      + One type whose size can't be known at compile time is a recursive type, where a value can
//      have as part of itself another value of the same type.
//      -> infinitely, Rust doesn't know how much space a value of a recursive type needs.
//
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    // The Cons variant will need the size of an i32 plus the space to store the box's pointer data
    // Smart pointer because it implements `Deref` trait, which allows `Box<T>` values to be
    // treated like references
    // Drop trait

    // =============================================================================================
    // Treating Smart Pointers Like Regular References with `Deref` Trait
    // =============================================================================================
    // allows you to customize the behavior of the deference operator `*`

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);


    // Use `Box<T> Like a reference`
    //
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(5, *y);

    println!("b = {}", b);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
    // behind the scences
    // *(y.deref());
    //
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Defining Our Own Smart Pointer

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // why use &T here?
        // because if the `deref` method returned the value directly instead of a reference to the
        // value, the value would be moved out of self.
        &self.0
    }
}

// Implicit Deref Coercions with Functions and Methods
// is a convenience that Rust performs on args to function and methods
// Convert a reference to a type that implements `Deref` into a reference to a type that `Deref`
// can convert the original type into.
// Happens automatically when we pass a reference to a particular type's value as an argument to a
// function or method that doesn't match the parameter type in the function or method definition.
//

fn hello(name: &str) {
    println!("Hello, {}", name);
    // fn main() {
    //     let m = MyBox::new(String::from("Rust"));
    //     hello(&m);
    //     as same as
    //     hello(&(*m)[..]); // hard to read
    // }
    //
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
}
