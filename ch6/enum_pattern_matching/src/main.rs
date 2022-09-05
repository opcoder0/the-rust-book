#[derive(Debug)]
enum IpAddrKindV1 {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKindV2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // like an unit struct
    Move { x: i32, y: i32 },    // like struct fields
    Write(String),              // string
    ChangeColor(i32, i32, i32), // tuple
}

impl Message {
    fn call(&self) {
        println!("Message {:#?}", self);
    }
}

// Instead of storing the IP Address in a struct this can be reversed and
// stored in an enum. Example -
//
// struct IpAddr {
//   kind: IpAddrKindV1,
//   address: String,
// }
//
// This can instead be stored in an Enum like IpAddrKindV2 -
//
// enum IpAddrKindV2 {
//    V4(String),
//    V6(String),
// }
//
// The Enum variants can have different types of values.
//
// enum IpAddrKindV3 {
//   V4(u8, u8, u8, u8),
//   V6(String),
// }
//

fn main() {
    let ipv4 = IpAddrKindV1::V4;
    let ipv6 = IpAddrKindV1::V6;
    println!(
        "IP Address (V1): v4 type => {:?}, v6 type => {:?}",
        ipv4, ipv6
    );

    let ipv4 = IpAddrKindV2::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddrKindV2::V6(String::from("::1"));
    println!(
        "IP Address (V2): v4 type => {:?}, v6 type => {:?}",
        ipv4, ipv6
    );

    let msg: Message = Message::Move { x: 0, y: 30 };
    msg.call();
    let msg: Message = Message::Quit;
    msg.call();
    let msg: Message = Message::ChangeColor(0, 0, 0);
    msg.call();
    let msg: Message = Message::Write(String::from("Rust goooood"));
    msg.call();
}
