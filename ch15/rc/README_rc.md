# Rc<T>, the reference counted smart pointer

Single threaded reference counting pointers. Rc provides shared ownership but in an immutable way. The `Rc` type doesn't implement the `Send` trait so when it is passed between threads the compiler catches it. Example of passing around Rc to functions for sharing -

```
use std::rc::Rc;

fn main() {
    let n = Rc::new(5);
    println!("created rc: {}", n);
    foo(Rc::clone(&n));
    println!("reference count (in main) = {}", Rc::strong_count(&n));
}

fn foo<T>(n: Rc<T>) 
where
    T: std::fmt::Display
{
    println!("received: {}", n);
    println!("reference count (in foo) = {}", Rc::strong_count(&n));
}
```

Output is -

```
created rc: 5
received: 5
reference count (in foo) = 2
reference count (in main) = 1
```

In majority of cases ownership is clear and a single value is owned by a single owner. However there are certain cases a single value is owned by multiple owners. For example the graph data structures multiple edges might point to the same node and owned by multiple edges.

Let's take the cons list as an example again. To represent a list like this one -

![Shared List](../../img/trpl15-03.svg)

You see there are two lists `b` and `c` point to the list `a`.

Implementing this with Box will result in errors -

```
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

```

Hence `Box<List>` can be changed to `Rc<List>`; This would let `b` and `c` to share `a`. The sharing is done by using `Rc::clone(&a)` instead of `a.clone()` which does deep copy. The `Rc::clone()` performs a shallow copy and increments the reference count. See the [example here](./shared_list/src/main.rs).

## Rc<T> and reference count

```
use std::rc::Rc;

fn main() {
    let five = Rc::new(5);
    let five_ptr = Rc::as_ptr(&five);
    println!("Rc (five): {}, address: {:#?}", five, five_ptr);
    let five_ref = Rc::clone(&five);
    let five_ptr = Rc::as_ptr(&five_ref);
    println!("Rc (five_ref): {}, address {:#?}", five_ref, five_ptr);
    // cloning increases reference count
    println!("number of references: {}", Rc::strong_count(&five));
}
```

Output -

```
Rc (five): 5, address: 0x0000563da171f9e0
Rc (five_ref): 5, address 0x0000563da171f9e0
number of references: 2
```

## Weak References

Weak Reference doesn't increment the strong count. A `Rc` that's downgraded returns a weak pointer that can be used via an `upgrade` call. `upgrade` returns an `Option` which has `Some` and if the value is dropped it returns `None`. See [weak reference](./weak_ref) code here.
