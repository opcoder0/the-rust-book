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
impl<T: Display + PartialOrd> Pair<T> {
    fn display(&self) {
        println!("Pair({}, {})", self.x, self.y);
    }
}

fn main() {
    let p1 = Pair::new(5, 10);
    print!("Displaying pair: ");
    p1.display();
}
