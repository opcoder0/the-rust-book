fn main() {
    let s1: String = String::from("hello");
    println!("s1 is {}", s1);
    // ownership is now moved to s2.
    // That way only one owner
    let s2 = s1;
    // cannot use s1 after the line above
    // compiler error - value borrowed/used after
    // move.
    println!("s2 is {}", s2);
}
