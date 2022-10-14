# RefCell<T> and the Interior Mutability Pattern

Interior Mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; this action is disallowed by the borrowing rules. To mutate data the pattern uses `unsafe` code inside the data structures to bend Rust's usual rules. We can use types that use interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime even though the compiler can't guarantee that.

Let's explore `RefCell<T>` and see how it enables us to implement the interior mutability pattern.

## Enforcing Borrowing Rules at Runtime using RefCell<T>

Unlike `Rc<T>` `RefCell<T>` is the single owner of the value. How does this then differ from `Box<T>`. While `Box<T>` enforces the borrowing rules at compile time, `RefCell<T>` enforces them at runtime.

**Borrowing Rules**

- At any time there can only be either 
  - One mutable reference or 
  - Any number of immutable references
- References must always be valid

When the above rules are broken `Box<T>` issues compiler errors while `RefCell<T>` raises a `panic!`.

The advantage of using `RefCell<T>` and enabling borrow check at runtime is that certain memory safe operations that are rejected by the compiler are allowed. 

Similar to `Rc<T>`, `RefCell<T>` is only for use in single threaded scenarios.

## Interior Mutability: A Mutable Borrow to an Immutable Value

The consequence of borrowing is that when you have an immutable value, you cannot borrow it mutably. For example -

```
fn main() {
    let x = 5;
    let y = &mut x;
}
```

Will give the error -

```
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:4:13
  |
3 |     let x = 5;
  |         - help: consider changing this to be mutable: `mut x`
4 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable
```

Use `RefCell<T>` to mutate an immutable object;

Usecases where this is useful is Mock Objects. Example of using mock objects can be seen 

- [Example-1](./example_1/src) (Without using RefCell<T> - throws error)
- [Example-2](./example_2/src) (Using RefCell<T> - fixes the issue)

[Example-1](./example_1/src) throws the following error upon running `cargo test` 

```
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:57:13
   |
2  |     fn send(&self, msg: &str);
   |             ----- help: consider changing that to be a mutable reference: `&mut self`
...
57 |             self.sent_messages.push(msg.to_string());
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `example_1` due to previous error
```

**NOTE** - The Messenger trait signature for `send` uses an immutable `&self` while the `MockMessenger` needs to update itself. And to overcome this issue and implement a mock messenger we use `RefCell<T>`.

[Example-2](./example_2/src) fixes the issue reported above. The `RefCell` has `borrow_mut()` method that can borrow a mutable reference from within an immutable method. 

## Keeping track of borrows at Runtime with RefCell<T>

Just like we use `&` and `&mut` for a reference and mutable reference, with `RefCell<T>` we use `borrow` and `borrow_mut`. `borrow` returns `Ref<T>` and `borrow_mut` returns `RefMut<T>`. These functions keeps track of how many immutable / mutable references are given out and panics when borrow checks fail.

## Having Mutable shared data using Rc<T> and RefCell<T> combination

A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!

`Rc<T>` lets you have multiple owners of some data immutably. `RefCell<T>` lets you have mutable data to a single owner. Combining them as shown below lets you have both -

```
let value = Rc::new(RefCell::new(5));
```

Example here - [./shared_mutability](./shared_mutability/src/main.rs)

**NOTE** `RefCell<T>` is not suitable for multithreaded programs. We use `Mutex<T>` for multithreaded code.
