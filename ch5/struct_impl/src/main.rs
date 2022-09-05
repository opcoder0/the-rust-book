#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of the rectangle {:?} is {} squares", r, r.area());

    let r1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("r {:?} can hold r1 {:?}: {}", r, r1, r.can_hold(&r1));

    let square = Rectangle::square(40);
    println!("square is a special rectangle {:?}", square);
}
