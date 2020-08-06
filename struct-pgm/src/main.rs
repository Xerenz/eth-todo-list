struct Rectangle {
    height : u32,
    width : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.height > r.height && self.width > r.width
    }

    // associated functions
    fn square(size : u32) -> Rectangle {
        Rectangle {
            height : size,
            width : size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height : 50,
        width : 30,
    };

    let rect2 = Rectangle {
        height : 10,
        width : 5,
    };

    let rect3 = Rectangle {
        height : 60, 
        width : 70,
    };

    let square1 = Rectangle::square(30);

    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );

    println!("r2 fits in r1 {}", rect1.can_hold(&rect2));

    println!("r3 fits in r1 {}", rect1.can_hold(&rect3));

}

