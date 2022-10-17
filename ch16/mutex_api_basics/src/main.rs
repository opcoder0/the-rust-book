use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut v = m.lock().unwrap();
        *v += 1;
    }

    println!("final value: {:?}", m);
}
