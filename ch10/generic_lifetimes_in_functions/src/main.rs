fn main() {
    let s1 = String::from("abcd");
    let s2 = "this is a long string";

    let result = longest(s1.as_str(), s2);
    println!("longer string: {}", result);
}

fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
