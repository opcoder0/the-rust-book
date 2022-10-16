use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = vec!["hi", "from", "the", "thread"];
        for s in msg {
            tx.send(s).unwrap();
        }
    });

    for msg in rx {
        println!("received in main: {}", msg);
    }
}
