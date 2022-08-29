fn main() {
    let s: String = String::from("hello world");

    // `h` and `w` are string slices.
    let h = &s[0..5];
    let w = &s[6..11];
    println!("=> s: {}, h => &s[0..5] {}, s => &s[6..11] {}", s, h, w);

    // slice uses
    let s: String = String::from("hello");
    let s1 = &s[0..2];
    let s2 = &s[..2];
    println!("=> s: {}, s1: {}, s2: {}", s, s1, s2);
    let s3 = &s[..];
    println!("=> s: {}, s3: {}", s, s3);

    // first_word gives the first word from space seprated
    // words
    let s: String = String::from("abacus machine");
    let first = first_word(&s);
    println!("full word: {}, first word end index: {}", s, first);
    println!("first word: {}", &s[0..first]);
    //
    // program runs beyond this point and crashes
    //
    // s.clear();
    // println!("first word: {}", &s[0..first]);

    // first_word_slice solves the problem above by returning the slice
    // and also stopping the crash error via compiler check
    //
    // let mut s: String = String::from("abacus machine");
    // let first = first_word_slice(&s);
    // println!("full word: {}, first word: {}", s, first);
    // s.clear();
    //
    //   Compiling slice_type_4 v0.1.0 (/home/saikiran/src/rust/the-rust-book/slice_type_4)
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //   --> src/main.rs:34:5
    //    |
    // 32 |     let first = first_word_slice(&s);
    //    |                                  -- immutable borrow occurs here
    // 33 |     println!("full word: {}, first word: {}", s, first);
    // 34 |     s.clear();
    //    |     ^^^^^^^^^ mutable borrow occurs here
    // 35 |     println!("first word: {}", first);
    //    |                                ----- immutable borrow later used here
    //
    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `slice_type_4` due to previous error
    // println!("first word: {}", first);

    let s: String = String::from("Ubuntu Focal");
    let first = first_word_slice(&s);
    println!("s: {}, first word: {}", s, first);

    generic_slices();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// changing signature to
// fn first_word_slice(s: &str) -> &str
// would allow the function to accept &String or &str (string literals/slices)
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn generic_slices() {
    let a: [u32; 5] = [2, 4, 6, 8, 10];
    let slice = &a[1..3];
    println!("arr: {:?}, slice: {:?}", a, slice);
}
