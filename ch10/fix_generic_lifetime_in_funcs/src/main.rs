fn main() {
    let s1 = String::from("abcd");
    let s2 = "this is a long string";
    let r = longest(s1.as_str(), s2);
    println!("longer string: {}", r);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
