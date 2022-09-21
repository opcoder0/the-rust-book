#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let mut list = vec![
        Rectangle {
            width: 4,
            height: 6,
        },
        Rectangle {
            width: 6,
            height: 3,
        },
        Rectangle {
            width: 8,
            height: 4,
        },
    ];
    let mut number_of_calls: i32 = 0;
    list.sort_by_key(|r| {
        number_of_calls += 1;
        r.height
    });
    println!("Sorted list: {:#?}", list);
    println!("Number of calls to key-fn {}", number_of_calls);
}
