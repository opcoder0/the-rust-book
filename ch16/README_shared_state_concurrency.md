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



