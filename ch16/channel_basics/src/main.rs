use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // using value here is a concurrency error and is caught by the
        // compiler using ownership rules.
        // println!("val: {}", val);
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
