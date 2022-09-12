#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn y(&self) -> &Y1 {
        &self.y
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 3.5, y: 4.5 };
    let q: Point<i32, i32> = Point { x: 4, y: 5 };

    println!("p: {:?}", p);
    let mixup = p.mixup(q);
    println!("mixup: {:?}", mixup);
}
