// const MAX_POINTS: u32 = 10000;

fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    println!("Max points = {}", MAX_POINTS);

    shadowing

    let x = 2;
    println!("x = {}", x);
    let x = x * x;
    println!("x = {}", x);
    let x = x + x;
    println!("x = {}", x);

    we can also change types using shadowing

    let spaces = "    ";
    let spaces = spaces.len();

    let tup: (i32, f32, char) = (1, 1.2, 'c');
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    let x = tup.0;

    println!("x = {}", x);

    let a = [1,2,3];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
