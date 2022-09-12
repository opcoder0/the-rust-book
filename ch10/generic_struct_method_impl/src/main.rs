#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 2.5, y: 3.5 };
    println!("Point p co-ordinates at ({}, {})", p.x(), p.y());

    let p32: Point<f32> = Point { x: 3.5, y: 6.5 };
    println!(
        "p32 {:?} distance from origin: {:?}",
        p32,
        p32.distance_from_origin()
    );
}
