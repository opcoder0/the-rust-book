fn main() {
    let s1 = String::from("hello world");
    let (s2, len) = calc_len_own(s1);
    println!("calc_len_own   : s2 => {}, len => {}", s2, len);
    let len = calc_len_borrow(&s2);
    println!("calc_len_borrow: s2 => {}, len => {}", s2, len);
    println!();
    let mut s3 = String::from("Hello, ");
    println!("change (before)  : s3 => {}", s3);
    change(&mut s3);
    println!("change (after)   : s3 => {}", s3);

    println!();
    println!("mutable references");
    mut_refs();

    println!();
    println!("mixed");
    mixed();
}

// method owns and returns ownership back via return
fn calc_len_own(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// method borrows instead of taking ownership
fn calc_len_borrow(s: &String) -> usize {
    // s is a reference to what is passed
    s.len()
} // s goes out of scope but does not drop what it points to

// modify by owning
fn change(s: &mut String) {
    s.push_str("World!");
}

// there can only be one mutable reference
// fn mut_ref() {
//     let mut s: String = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}, {}", r1, r2);
// }
// Compiling refs_borrowing_4 v0.1.0 (/home/saikiran/src/rust/the-rust-book/refs_borrowing_4)
// error[E0499]: cannot borrow `s` as mutable more than once at a time
//   --> src/main.rs:38:14
//    |
// 37 |     let r1 = &mut s;
//    |              ------ first mutable borrow occurs here
// 38 |     let r2 = &mut s;
//    |              ^^^^^^ second mutable borrow occurs here
// 39 |     println!("{}, {}", r1, r2);
//    |                        -- first borrow later used here
//
// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `refs_borrowing_4` due to previous error

fn mut_refs() {
    let mut s: String = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1: {}", r1);
}

// cannot have a mutable and immutable references
// mixed up
// fn nomix() {
//     let mut s1 = String::from("hello");
//     let r1 = &s1; // OK
//     let r2 = &s1; // OK
//     let r3 = &mut s1; // NOT OK
//     println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
// }
// Compiling refs_borrowing_4 v0.1.0 (/home/saikiran/src/rust/the-rust-book/refs_borrowing_4)
// error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
//   --> src/main.rs:73:14
//    |
// 71 |     let r1 = &s1; // OK
//    |              --- immutable borrow occurs here
// 72 |     let r2 = &s1; // OK
// 73 |     let r3 = &mut s1; // NOT OK
//    |              ^^^^^^^ mutable borrow occurs here
// 74 |     println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
//    |                                        -- immutable borrow later used here
//
// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `refs_borrowing_4` due to previous error

// Cannot have mutable and immutable references to value (see above)
// immutable references cannot be used after a mutable reference
// has been created (see below; acceptable)
fn mixed() {
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);

    let r3 = &mut s1;
    println!("r3: {}", r3);
}
