#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quater(USStates),
}

#[derive(Debug)]
enum USStates {
    Alabama,
    California,
}

fn main() {
    value_in_cents(Coin::Quater(USStates::Alabama));
    let five = Some(5);
    println!("Value = {:?}", plus_one(five));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("This belongs to the state {:?}", state);
            25
        }
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn get_some_number(i: Option<u8>) {
    match i {
        Some(1) => println!("one"),
        Some(2) => println("two"),
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        _ => (), // handle all other cases
    }
}

fn handle_one_case(i: Option<u8>) {
    if let Some(3) = i {
        println("three");
    }
}
