# Fearless Concurrency

Rust's ownership rules and strong type system help Rust managing memory safety and _concurrency problems_. So by leveraging this Rust is able to catch lots of concurrency problems at compile time. This is nicknamed in Rust as _Fearless Concurrency_.

### What is covered 

- [How to create threads to run multiple pieces of code at the same time](./README_thread_basics.md)
- [_Message passing_ concurrency where channels send messages between threads](./README_channels.md)
- [_Shared state_ concurrency where multiple threads have access to some piece of data](./README_shared_state_concurrency.md)
- The `Sync` and `Send` traits which extend Rust's concurrency guarantees to user defined types as well as those provided in the standard library.


