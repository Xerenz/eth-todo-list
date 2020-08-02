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