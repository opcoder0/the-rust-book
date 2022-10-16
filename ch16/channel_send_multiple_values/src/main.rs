use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = vec!["hi", "from", "the", "thread"];
        for s in msg {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("received in main: {}", msg);
    }
    println!("done: main end");
}
