# Fearless Concurrency

Rust's ownership rules and strong type system help Rust managing memory safety and _concurrency problems_. So by leveraging this Rust is able to catch lots of concurrency problems at compile time. This is nicknamed in Rust as _Fearless Concurrency_.

### What is covered 

- How to create threads to run multiple pieces of code at the same time.
- _Message passing_ concurrency where channels send messages between threads.
- _Shared state_ concurrency where multiple threads have access to some piece of data.
- The `Sync` and `Send` traits which extend Rust's concurrency guarantees to user defined types as well as those provided in the standard library.

## Using Threads to run code simultaneously

Threads can run simultaneously and the order in which they run is not guaranteed. This can lead to certain problems -

- Race conditions - threads accessing shared resources in an inconsistent manner
- Deadlocks - Threads waiting indefinitely on eachother to access a resources
- Bugs that happen only under certain circumstances that are hard to reproduce.

Rust tries to mitigate some of the negative effects of using threads. But programming with threads still takes careful thought and requires a code structure that is different from a single threaded program.

**NOTE -**

Rust implements a 1:1 thread model. Where one thread created from the Rust programming language maps to one operating system thread. There are other library crates available with other thread models.

## Creating a new thread with spawn

The module `std::thread` has thread related calls. Thread can be spawned using `thread::spawn` function which accepts a closure as its argument. Example -

```
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

**NOTE -** Once the main thread / program terminates all other threads are terminated too.

`thread::spawn` forces to stop the execution of thread for a short duration allowing a different thread to run.

The above program terminates the thread prematurely.

## Waiting for all threads to finish using join handles

A thread that is spawned returns a handle (`JoinHandle`) on which `join` method can be called to synchronize thread execution. Example [join threads](./join_thread). Call to `join` waits for the thread identified by the handle to finish at the calling point before proceeding.
