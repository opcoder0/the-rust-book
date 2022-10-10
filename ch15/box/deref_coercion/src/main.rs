use std::ops::Deref;

#[derive(Debug, PartialEq)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn decorate_print(s: &str) {
    let decorator = "*";
    println!("{}", decorator.repeat(s.len()));
    println!("{}", s);
    println!("{}", decorator.repeat(s.len()));
}

fn main() {
    let s = MyBox::new(String::from("hello world"));
    decorate_print(&s);
}
