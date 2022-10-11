use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }
//
// The above will give errors because the Box will own the list and cannot be shared.
//
//
// let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//    |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
// 10 |     let b = Cons(3, Box::new(a));
//    |                              - value moved here
// 11 |     let c = Cons(4, Box::new(a));
//    |                              ^ value used here after move

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(v, next) => {
                write!(f, "{}, ({})", v, next)
            }
            Nil => {
                write!(f, "Nil")
            }
        }
    }
}

use crate::List::Cons;
use crate::List::Nil;

fn main() {
    let a = Rc::new(Cons(
        1,
        Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil))))))),
    ));

    let b = Rc::new(Cons(5, Rc::clone(&a)));
    let c = Rc::new(Cons(6, Rc::clone(&a)));

    println!("a: {:#?}", a);
    println!("b: {:#?}", b);
    println!("c: {:#?}", c);
}
