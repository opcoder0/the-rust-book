static GLOBAL_STATIC: &str = "This is a global static string";

fn main() {
    let s: &'static str = "This is a local static string";
    println!("static string: {}", s);
    println!("global static: {}", GLOBAL_STATIC);
}
