use std::cmp::PartialOrd;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditionally implement methods based on the type of trait
//
impl<T: Display + PartialOrd> Pair<T> {
    fn display(&self) {
        if self.x > self.y {
            println!("pair x {} < pair y => {}", self.x, self.y);
        } else {
            println!("pair y {} < pair x => {}", self.y, self.x);
        }
    }
}

fn main() {
    let p1 = Pair::new(5, 10);
    println!("DISPLAY");
    p1.display();
}
