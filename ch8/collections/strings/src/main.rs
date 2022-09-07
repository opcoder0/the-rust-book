fn main() {
    // create a new empty string
    let _s: String = String::new();
    let _s: String = String::from("hello world");
    let data: &str = "hello world";
    let s: String = data.to_string();
    println!("data: {}, string: {}", data, s);

    // update the string
    let mut s: String = String::from("String is");
    s.push_str(" stored on the heap;");
    s = s + " It is implemented as a vector of bytes";
    println!("string: {}", s);
    s.push('.');
    println!("string: {}", s);

    // concat
    let s1: String = String::from("For rust internals");
    let s2: String = String::from(" read the book");
    // below s1 is moved so cant be used after this line
    let s3: String = s1 + &s2;
    println!("{}", s3);

    // format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    let s5 = "Здравствуйте";
    let first_grapheme = &s5[0..2];
    println!("First letter: {}", first_grapheme);
    //
    // Rust panics if the indexing does not fit the grapheme cluster
    //
    // let wrong_index = &s5[0..1]; // panics
    //
    // thread 'main' panicked at 'byte index 1 is not a char boundary;
    // it is inside 'З' (bytes 0..2) of `Здравствуйте`',
    // library/core/src/str/mod.rs:127:5
    // note: run with `RUST_BACKTRACE=1` environment variable to
    // display a backtrace
    //

    // iterating over strings (&str)
    let s5 = "Здравствуйте";
    println!("iterating chars of {}", s5);
    for c in s5.chars() {
        println!(" => {}", c);
    }

    let s5 = "Здр";
    println!("iterating bytes of {}", s5);
    print!("[ ");
    for b in s5.bytes() {
        print!("{} ", b);
    }
    println!("]");
}
