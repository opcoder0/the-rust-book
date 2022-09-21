#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = vec![
        Rectangle {
            width: 2,
            height: 3,
        },
        Rectangle {
            width: 4,
            height: 5,
        },
        Rectangle {
            width: 1,
            height: 2,
        },
        Rectangle {
            width: 3,
            height: 4,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("Sorted by keys: ");
    println!("{:#?}", list);
}
