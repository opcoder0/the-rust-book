fn main() {
    regular_ref();
    boxed_ref();
}

fn regular_ref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x: {}, y: {}", x, y); // y is de-referenced here automatically

    // assert_eq!(x, y);
    // no implementation for `{integer} == {&integer}`
}

fn boxed_ref() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x: {}, y: {}", x, y); // y is de-referenced here automatically

    // assert_eq!(x, y);
    // no implementation for `{integer} == Box<{integer}>`
}
