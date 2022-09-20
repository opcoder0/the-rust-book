use std::thread;
use std::time::Duration;

fn main() {
    let expensive_function = |num: u32| {
        println!("performing expensive calculation");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = expensive_function(2);
    println!("expensive function: {}", x);

    add_one();
}

fn add_one() {
    // add_one_v1 is a regular function
    // fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let y = 0;
    println!("add_one_v1({}) => {}", y, add_one_v1(y));
    println!("add_one_v2:{} => {}", y, add_one_v2(y));
    println!("add_one_v3:{} => {}", y, add_one_v3(y));
    println!("add_one_v4:{} => {}", y, add_one_v4(y));
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}
