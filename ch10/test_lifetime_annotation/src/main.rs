fn main() {
    let s1 = String::from("this is a long string");
    let result;
    {
        let s2 = String::from("abcd");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("longer string is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
