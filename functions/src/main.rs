fn main() {
    let x = 6;

    let y = {
        let x = x + 2;
        x + 1
    };

    let y = plus_one(y);

    println!("y = {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
