enum MessageType {
    Json,
    XML,
    Text,
    Binary,
}

#[derive(Debug)]
enum Message {
    Quit,                       // like an unit struct
    Move { x: i32, y: i32 },    // like struct fields
    Write(String),              // string
    ChangeColor(i32, i32, i32), // tuple
}

fn main() {
    let some_number: Option<i32> = Some(20);
    println!("some_number: {:?}", some_number);

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    // let sum = x + y;
    // --> src/main.rs:7:17
    //   |
    // 7 |     let sum = x + y;
    //   |                 ^ no implementation for `i8 + Option<i8>`

    println!("Enum match expression");
    let m = MessageType::Json;
    match m {
        MessageType::Json => {
            println!("JSON Message");
        }
        MessageType::XML => {
            println!("XML Message");
        }
        MessageType::Text => {
            println!("Text Message");
        }
        MessageType::Binary => {
            println!("Binary Message");
        }
    }

    // extract value from Enum using match

    println!("Enum extract value using match");
    let m = Message::Move { x: 0, y: 90 };
    match m {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => {
            println!("Move: x {}, y {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color change to ({}, {}, {})", r, g, b);
        }
    }

    let n = Some(5);
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);

    let n: Option<i32> = None;
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}
