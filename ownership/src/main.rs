fn main() {

    // immutable data types reside in stack
    let x = 5;
    let y = x;

    // incase of stack allocation full data is copied 
    let s1 = "hello";
    let s2 = s1;

    // using String
    let s3 = String::from("hello again"); // runtime assignment happens
    // s3 has the pointer to heap, length and capacity in stack
    // value "hello" is stored in heap
    // let s4 = s3; // this invalids s3 and only s4 is valid
    // this is to avoid double memory deallocation when var out of scope

    let s4 = s3.clone(); // produce deep copy of string in runtime

    println!("x = {}, y = {}", x, y);
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s3 = {}, s4 = {}", s3, s4); // s3 and s4 can be used here

    take_ownership(s3); // ownership of s3 is tranferred
    // println!("s3 is out of ownership {}", s3); // value borrowed here after moved
    println!("s4 is still owned {}", s4);
    take_ownership(s4); // s4 is gone

    make_copy(x); // a copy of x is transfered

    println!("You still have me here x = {}", x);

    let s3 = give_ownership();

    println!("I can still use s3 = {}", s3);

    let s4 = take_and_give_ownership(s3);

    println!("I can no longer use s3 but s4 = {}", s4);

    let s3 = String::from("Ok I made a new string here");

    let (s3, len) = return_string_and_length(s3);

    println!("The length of string {} is {}", s3, len);

    println!("length of string {}", get_length(&s3));

    let mut s4 = String::from("Some string");
    modify_string(&mut s4);

    println!("modified string = {}", s4);

    // code can have any number of immutable references
    // but only 1 mutable reference

    let x = 5;
    let r1 = &x;
    let r2 = &x;

    println!("r1 = {}, r2 = {}", r1, r2);

    let mut z = 5;
    let r3 = &z; // possible
    // to have another reference add new scope
    {
        let r4 = &z;
        println!("r4 = {}", r4);
    }
    // no r4 after scope
    println!("r3 = {}", r3);
}

fn take_ownership(s: String) {
    println!("I take ownership of {}", s);
}

fn make_copy(n: i32) {
    println!("I simply made a copy of {}", n);
}

// returning functions give their ownership
// or bring ownership to another thread

fn give_ownership() -> String {
    let s = String::from("I give myself to you");
    return s;
}

fn take_and_give_ownership(s1: String) -> String {
    println!("I took ownership of s1 = {}", s1);
    return s1;
}

fn return_string_and_length(s: String) -> (String, usize) {
    let len: usize = s.len(); // should always be usize
    return (s, len);
}

// using referencing
fn get_length(s: &String) -> usize {
    s.len()
}

// mutable referencing
fn modify_string(s: &mut String) {
    s.push_str(", this new string got modified");
}


// &str 
fn first_word(s: &str) -> &str {
    let s = s.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}