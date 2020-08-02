use std::io;

fn main() {
    let result = true;

    let x = if result { 5 } else { 6 };
    // invalid : let x = if result { 5 } else { "six" };
    println!("x = {}", x);

    returning a result from a loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result = {}", result);

    while loop 

    let mut number = 3;

    while number != 0 {
        number -= 1;

        println!("{}", number);
    }

    let a = [1, 2, 3, 4, 5];

    let mut i = 0;

    while i < 5 {
        println!("{}", a[i]);
        i += 1;
    }

    using for loop

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("{}", element);
    }

    for i in (1..11).rev() {
        println!("{}", i);
    }
    println!("LIFTOFF!!!");

    let mut celcius = String::new();

    println!("Enter temperature in celcius");

    io::stdin()
    .read_line(&mut celcius)
    .expect("Failed to read line");

    let celcius: i32 = celcius.trim().parse()
                        .expect("Enter valid number");

    let fahrenheit = fahrenheit_to_celsius(celcius);

    println!("{} C = {} F", celcius, fahrenheit);

    let mut n = String::new();

    println!("Enter number");

    io::stdin()
    .read_line(&mut n)
    .expect("failed to read line");

    let n: i32 = n.trim().parse()
                .expect("enter valid number");

    let nth_fibo = fibonacci(n);

    println!("{}th fibonacci is {}", n, nth_fibo);

    twelve_days_of_christmas()
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

fn twelve_days_of_christmas() {
    let gifts = [
        "12 drummers drumming\n",
        "Eleven pipers piping\n",
        "Ten lords a leaping\n",
        "Nine ladies dancing\n",
        "Eight maids a milking\n",
        "Seven swans a swimming\n",
        "Six geese a laying\n",
        "Five gold rings, badam-pam-pam\n",
        "Four calling birds\n",
        "Three French hens\n",
        "Two turtle doves\n",
        "And a partridge in a pear tree\n"
    ];

    let days = [
        "first", 
        "second", 
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelveth"
    ];

    let mut presents = String::new();
    let mut index = 11;

    for i in 0..12 {
        let mut gift = gifts[index].to_string();
        gift.push_str(&presents);
        presents = gift;

        println!("On the {} of Christmas", days[i]);
        println!("My true love gave me");
        println!("{}\n\n", presents);
        index -= 1;
    }
}
