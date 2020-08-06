// use std::io;

struct User {
    username : String,
    email : String,
    sign_in_account : u64,
    active : bool,
}

// tuple structs
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main() {
    let mut user = User {
        username : String::from("Martin George"),
        email : String::from("example@xyz.com"),
        sign_in_account : 1,
        active : true,
    };

    println!("{}", user.username);

    user.email = String::from("change@xyz.com");

    // struct update syntax
    let new_user = User {
        username : String::from("Another Name"),
        email : String::from("another@xyz.com"),
        ..user
    };

    let black = Color(0, 0, 0); // usefull when you want 
    let origin = Point(0, 0, 0); // different values 
}

fn build_user(username : String, email : String) -> User {
    User {
        username,
        email,
        sign_in_account : 1,
        active : true,
    }
}
