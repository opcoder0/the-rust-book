fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("Before declaring closure: {:?}", v1);
    let mut mutable_borrow = || v1.push(5);
    mutable_borrow();
    println!("After calling closure {:?}", v1);
}
