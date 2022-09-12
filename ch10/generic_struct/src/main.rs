#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointV2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let zero = Point { x: 0, y: 0 };
    println!("zero point: {:?}", zero);

    let start = PointV2 { x: 4, y: 5.5 };
    let end = PointV2 { x: 2.5, y: 9 };
    println!("start: {:?}, end: {:?}", start, end);
}
