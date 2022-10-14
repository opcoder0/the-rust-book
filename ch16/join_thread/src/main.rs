use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().expect("could not synchronize thread");

    for i in 1..5 {
        println!("hi number {} from main!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().expect("could not synchronize thread");
}
