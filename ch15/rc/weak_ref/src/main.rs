use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped");
    }
}

fn main() {
    let foo = Foo {};
    let foo_rc = Rc::new(foo);
    let _weak_rc_1 = Rc::downgrade(&foo_rc);
    let _weak_rc_2 = Rc::downgrade(&foo_rc);
    println!(
        "strong_count: {}, weak_count: {}",
        Rc::strong_count(&foo_rc),
        Rc::weak_count(&foo_rc)
    );

    let v = Rc::new(vec![1, 2, 3, 4, 5]);
    foo_vec_strong(Rc::clone(&v));
    let weak_vec_1 = Rc::downgrade(&v);
    let weak_vec_2 = Rc::downgrade(&v);
    drop(v);
    foo_vec_weak(weak_vec_1);
    foo_vec_weak(weak_vec_2);
}

fn foo_vec_strong(v: Rc<Vec<i32>>) {
    println!("strong foo_vec");
    for i in v.iter() {
        println!("i = {}", i);
    }
}

fn foo_vec_weak(v: Weak<Vec<i32>>) {
    println!("weak foo_vec");
    let x = v.upgrade();
    match x {
        Some(vv) => {
            println!("something is there");
            for i in vv.iter() {
                println!("i = {}", i);
            }
        }
        None => {
            println!("nothing is there");
        }
    }
}
