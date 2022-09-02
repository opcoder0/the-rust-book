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

    println!("Enum with no attached values and using match expression");
    let json = MessageType::Json;
    enum_match_no_values(json);
    let xml = MessageType::XML;
    enum_match_no_values(xml);
    let txt = MessageType::Text;
    enum_match_no_values(txt);
    let bin = MessageType::Binary;
    enum_match_no_values(bin);

    // extract value from Enum using match
    println!("Enum attached with value and using match to extract");
    let m: Message = Message::Move { x: 0, y: 0 };
    enum_match_values(m);
    let q: Message = Message::Quit;
    enum_match_values(q);
    let w: Message = Message::Write(String::from("Hello"));
    enum_match_values(w);
    let c: Message = Message::ChangeColor(0, 0, 0);
    enum_match_values(c);

    // Option enum basics
    let n = Some(5);
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);

    let n: Option<i32> = None;
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);

    if_let();

    println!("if let else syntax");
    let m: Option<Message> = Some(Message::Move { x: 4, y: 4 });
    if_let_else(m);
    let n: Option<Message> = None;
    if_let_else(n);
}

fn enum_match_no_values(m: MessageType) {
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
}

fn enum_match_values(m: Message) {
    match m {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move: x = {}, y = {}", x, y),
        Message::Write(s) => println!("String: {}", s),
        Message::ChangeColor(r, g, b) => println!("Color {}, {}, {}", r, g, b),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn if_let() {
    let config_max = Some(3u8);

    // match expression
    println!("match syntax");
    match config_max {
        Some(max) => println!("Configured max {}", max),
        None => (),
    }

    // same using if let
    println!("if let syntax");
    if let Some(max) = config_max {
        println!("Configured max {}", max);
    }
}

fn if_let_else(m: Option<Message>) {
    if let Some(point) = m {
        println!("point: {:?}", point);
    } else {
        println!("no point");
    }
}
