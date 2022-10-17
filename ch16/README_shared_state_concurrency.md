# Shared-State Concurrency

Message passing is a fine way to handle concurrency but not the only way. Message passing (for example by using channels) show single ownership. i.e. once the data is sent over the channel it is no longer accessed or modified.

Shared-memory concurrency can have multiple owners that can access the same memory/data simultaneously.

## Using mutexes to allow access to data from one thread at a time

Mutex (_mutual exclusion_) usually owned by the shared data structure allows the owner of the mutex to modify the data. With Rust's ownership and type system you can't get locking and unlocking wrong.

## The API of Mutex<T>

To access the value inside a mutex we use `lock` method. Example [mutex api example](./mutex_api_basics/src/main.rs); 

**IMPORTANT NOTES**

- **NOTE-1** When a mutex is locked another call to `lock` will block. If the thread holding the lock panics the mutex isn't released. And our call to `lock` is never unblocked. To avoid this we can call `unwrap` to panic our thread instead.

- **NOTE-2** The type system ensures that we acquire the `lock` before using the value.

- **NOTE-3** Mutex<T> is a smart pointer. More accurately the call to `lock` returns pointer called `MutexGuard` wrapped with `LockResult` that we handled with `unwrap()`. The `MutexGuard` smart pointer implements the `Deref` trait that point to our inner data. The smart pointer also implements the `Drop` trait that allows for dropping of `MutexGuard` at the end of its scope and that automatically unlocks the mutex.

```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut v = m.lock().unwrap(); // returns MutexGuard
        *v += 1;
    }   // MutexGuard goes out of scope here and hence unlocks the mutex automatically.

    println!("final value: {:?}", m);
}
```

## Sharing a Mutex<T> Between Multiple Threads

Let's share a value among 10 threads.

The [example-1](./mutex_shared_across_multiple_threads_1/src/main.rs) throws a compiler error that says the value `counter` was `move`-d in the previous iteration. Rust detects that the value is being moved into multiple threads.

```
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            *v += 1;
        });
        handles.push(handle);
    }
    ...
```

```
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop

```

To fix this we use the `Rc<T>` which allows us to share values as shown in [example - 2](./mutex_shared_across_multiple_threads_2/src/main.rs) by -

```
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            *v += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
```

Here Rust detects that `Rc<T>` is being used by multiple threads and `Rc<T>` is not thread safe. As it does not implement `Send` trait. It throws the error -

```
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` cannot be sent between threads safely
12  | |             let mut v = counter.lock().unwrap();
13  | |             *v += 1;
14  | |         });
    | |_________- within this `[closure@src/main.rs:11:36: 14:10]`
    |
    = help: within `[closure@src/main.rs:11:36: 14:10]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
   --> src/main.rs:11:36
    |
11  |           let handle = thread::spawn(move || {
    |  ____________________________________^
12  | |             let mut v = counter.lock().unwrap();
13  | |             *v += 1;
14  | |         });
    | |_________^
note: required by a bound in `spawn`
   --> /home/saikiran/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:653:8
    |
653 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

```

## Using Arc<T> instead of Rc<T> to share across multiple threads

Fortunately the above problem can be solved with `Arc<T>` which stands for _atomic_ reference count. This one is thread safe as show in [example - 3](./mutex_shared_across_multiple_threads_3/src/main.rs)
