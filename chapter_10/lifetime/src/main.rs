use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 
// fn longest<'a>(x: &str, y: &str) -> &a' str {
//     let result = String::from("really long string");
//     result.as_str()
// }
//
// return value lifetime is not related to the lifetime of the parameters at all
//

// Lifetime Annotations in Struct Definition
//
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Three rules
// + each parameter that is a reference gets its own lifetime parameter
// + If there is exactly one input lifetime parameter, that lifetime is assigned to all output
// lifetime parameter
// + If there are multiple input lifetime parameter, but one of them is &self, or &mut self, the
// lifetime of self is assigned to all output lifetime parameter.
//
// fn first_word(s: &str) -> &str {}
// first rule
// fn first_word<'a>(s: &'a str) -> &'a str {}
//
// fn longest(x: &str, y: &str) -> &str {}
// first rule
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
//

// Lifetime Annotations in Method Definition
//
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Static lifetime
//
// let s: &'static str = "I have a static lifetime";
//
// The text of string is stored directly in the program's binary, which is always available

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

