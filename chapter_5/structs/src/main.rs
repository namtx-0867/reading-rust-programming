fn main() {
    let email = String::from("tran.xuan.nam@framgia.com");
    let username = String::from("namtx");

    let mut user = build_user(email, username);
    user.email = String::from("tran.xuan.nam@sun-asterisk.com");
    println!("{}", user.email);

    // update
    let another_user = User {
        email: String::from("another@framgia.com"),
        username: String::from("anotheruser4566"),
        ..user
    };

    println!("{}", another_user.email);

    // tuple struct
    let black = Color(0, 0, 0);

    // ownership
    let ownership_user = OwnershipUser {
        email: "someone@example.com",
        username: "someone",
        sign_in_count: 1,
        active: true
    };
}

struct Color(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct OwnershipUser {
    email: &str, 
    username: &str,
    sign_in_count: u64, 
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
