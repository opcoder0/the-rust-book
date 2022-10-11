use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::Cons;
use crate::List::Nil;

fn main() {
    let a = Rc::new(Cons(0, Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))))));
    println!("reference count of a: {}", Rc::strong_count(&a));
    {
        let b = Cons(3, Rc::clone(&a));
        println!(
            "reference count of a after b refers: {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "reference count of a after b refers: {}",
        Rc::strong_count(&a)
    );
}
