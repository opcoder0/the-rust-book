fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x: {}, y: {}", x, y); // y is de-referenced here automatically
                                    // assert_eq!(x, y); // not allowed as there is a type mismatch i32 == &i32
}
