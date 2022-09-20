fn main() {
    let list = vec![1, 2, 3, 4];
    println!("Before defining closure: {:?}", list);
    let borrow_immutably = || println!("from closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    borrow_immutably();
    println!("After calling closure {:?}", list);
}
