#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let mut list = vec![
        Rectangle {
            width: 5,
            height: 10,
        },
        Rectangle {
            width: 3,
            height: 9,
        },
        Rectangle {
            width: 8,
            height: 8,
        },
    ];
    let value = String::from("sort key function has been called");
    let mut counter: Vec<String> = Vec::new();
    list.sort_by_key(|r| {
        counter.push(value);
        r.width
    });
}
