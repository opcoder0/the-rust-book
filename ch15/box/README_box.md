# Box<T>

The most straight forward smart pointer is a _box_ written as `Box<T>`. Boxes allow you to store data on the heap rather than the stack. Boxes don't have performance overhead other than the fact that the value is on the heap.

Situations where `Box<T>` is used -

- When you have a type whose size is not known at compile time but want to use a value of the type in a context that requires exact size.
- When you have large amount of data and want to transfer ownership but don't want to copy the entire data over.
- When you want to own a value and you only care that it's type only implements a particular trait rather than being a specific type. (more on this in chapter 17 when discussing traits).

## Basic Syntax

```
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

When `b` goes out of scope the `b` and what `b` points to are both dropped (deallocated). 

## Times when Box<T> is useful

### Enabling Recursive Types with Boxes

A value of recursive types can have another value of the same type as part of itself. Recursive types pose an issue at compile time Rust needs to know the space it takes up and it cannot be computed. Because boxes have a known size we can make recursive types by inserting a `Box` in the recursive type definition.

```
enum List {
    Con(i32, List),
    Nil,
}
```

This is a recursive definition to which the compiler detects a cycle and cannot compute the size. This can be boxed instead -

```
enum List {
    Con(i32, Box<List>),
    Nil,
}
```

## Treating Smart Pointers Like Regular References with the Deref Trait

Implementing the `Deref` trait allows you to customize the `*` dereference operator. By implementing the `Deref` trait the smart pointer can be treated as a regular reference.

### Regular References

```
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Box References

```
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Implementing our own Box like smart pointer

In this example - 

```
#[derive(Debug, PartialEq)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);

    // assert_eq!(*y, 5);
    // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
}
```

The value in `y` (of type `MyBox<i32>`) cannot be dereferenced. 
As it does not implement the `Deref` trait.
