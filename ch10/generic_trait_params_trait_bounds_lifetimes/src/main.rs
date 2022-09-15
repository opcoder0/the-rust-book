use std::fmt::Display;

fn main() {
    let x = "tcm";
    let y = "the common man";
    let annotation = "news: This week's crate is bstr, a fast and featureful byte-string library.";
    let result = longest_with_announcement(x, y, annotation);
    println!("result: {}", result);
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, annotation: T) -> &'a str
where
    T: Display,
{
    println!("announcement: {}", annotation);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
