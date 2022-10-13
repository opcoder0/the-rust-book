use std::rc::Rc;

fn main() {
    let n = Rc::new(5);
    println!("created rc: {}", n);
    foo(Rc::clone(&n));
    println!("reference count (for n in main) = {}", Rc::strong_count(&n));

    // holding shared reference to a vector
    let shared_vec = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("created shared_vec: {:?}", shared_vec);
    foo(Rc::clone(&shared_vec));
    println!(
        "reference count (for shared_vec in main) = {}",
        Rc::strong_count(&n)
    );
}

fn foo<T>(n: Rc<T>) {
    // println!("received: {:?}", n);
    println!("reference count (in foo) = {}", Rc::strong_count(&n));
}
