fn main() {
    let s1: String = String::from("hello");
    println!("s1 is {}", s1);
    // ownership is now moved to s2
    let s2 = s1;
    // cannot use s1 after the line above
    // compiler error - value borrowed/used after
    // move.
    println!("s2 is {}", s2);

    scoped_ownership_move();

    // ownership is transferred when passing parameters
    let (s2, len) = word_length(s2);
    println!("Word {s2} has {len} bytes");

    let mut s2 = String::from("Hello ");
    println!("before append: {s2}");
    append_world(&mut s2);
    println!("after  append: {s2}");
}

fn scoped_ownership_move() {
    let s1: String = String::from("scoped: hello");
    println!("s1: {}", s1);
    {
        let s2: String = s1;
        println!("s2: {}", s2);
    }
    println!("done");
}

fn word_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn append_world(s: &mut String) {
    s.push_str("world")
}
