# Extensible Concurrency with Sync and Send Traits

Interestingly Rust language has very few concurrency features. Almost every concurrency feature we have discussed so far is part of the standard library, not the language. The language talks about concurrency using two traits from `std::marker`. They are `Sync` and `Send` traits.

## Allow transference of ownership between threads using Send trait

The `Send` trait marks that the ownership of values implementing the `Send` trait can be transferred between threads. Almost every Rust type implements `Send` trait. All primitive types implement the `Send` trait except for raw pointers. A composite type with all its members implementing `Send` trait is also considered to implement `Send` trait.

## Allowing access from multiple threads with Sync

The `Sync` trait marks that the type implementing the `Sync` trait is safe to be shared between multiple threads. In other words for any type `T` is `Sync` if `&T` (immutable reference) implements `Send`. A composite type with all its members implementing `Sync` the composite type is also considered to implement `Sync`.

- `Rc<T>` is not `Sync`
- `RefCell<T>` is not `Sync` - the runtime borrow checking rules implemented in RefCell are not thread safe
- `Mutex<T>` is `Sync`

## Implementing Sync and Send Manually is Unsafe

Any type that contains all `Sync` or `Send` traits is also considered `Sync` or `Send` safe. It is considered Unsafe to implement `Sync` or `Send` manually.
