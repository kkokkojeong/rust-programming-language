#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 재정의 가능
// Multiple impl Blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn method_syntax() {
    println!("-----method_syntax------");

    let rect1 = Rectangle {
        width: 5,
        height: 5
    };

    let sq1 = Rectangle::square(5);
    let sq2 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:?}", sq1);
    println!("{:?}", sq2);
}