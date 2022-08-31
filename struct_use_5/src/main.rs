#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut r = Rectangle {
        height: 36,
        width: 24,
    };
    // dbg! macro can be used to print values to stderr
    // The macro takes ownership and returns it back.
    println!("{:#?}", r);
    r = dbg!(r);
    dbg!("Area: {}", area(&r));
}

fn area(r: &Rectangle) -> u32 {
    r.height * r.width
}
