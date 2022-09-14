struct _ImportantExpert<'a> {
    data: &'a str,
}

struct _LinkedList<'a> {
    data: i32,
    next: &'a _LinkedList<'a>,
}

fn main() {}
