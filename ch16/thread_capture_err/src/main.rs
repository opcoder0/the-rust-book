use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("v from thread is: {:?}", v);
    });

    handle.join().unwrap();
}