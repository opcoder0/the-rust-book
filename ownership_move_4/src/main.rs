fn main() {
    let s1: String = String::from("hello");
    println!("s1 is {}", s1);
    let s2 = s1; // ownership is now moved to s2 (s2 <- s1)
                 // cannot use s1 after the line above
                 // compiler error - value borrowed/used after
                 // move.
    println!("s2 is {}", s2);

    let s3: String = String::from("hello");
    takes_ownership(s3); // s3 is unusable after this line

    let x: u32 = 1024;
    makes_copy(x);

    let (s2, len) = word_length(s2);
    println!("Word {s2} has {len} bytes");

    let s4 = gives_ownership();
    println!("gives_ownership: s4: {}", s4);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn makes_copy(x: u32) {
    println!("makes_copy: {}", x);
}

// ownership is transferred to s when calling this
// function.
fn word_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn gives_ownership() -> String {
    String::from("hello world")
}
