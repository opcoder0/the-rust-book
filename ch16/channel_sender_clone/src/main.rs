use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let v1 = ["this", "is", "from", "thread-1"];
        for msg in v1 {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v1 = ["more", "new", "message", "from", "thread-2"];
        for msg in v1 {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("recv: {}", msg);
    }
}
