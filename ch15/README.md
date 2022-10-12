# Smart Pointers

Pointers are a general concept. The most common kind of pointer in Rust is a reference. They are indicated using the symbol `&` and they borrow the value. Smart pointers on the other hand are data structures which store metadata and have additional capabilities. 

In Rust the difference between references and smart pointers - while references only borrow data; smart pointers own the data they point to.

Smart pointers are usually implemented as structs. Unlike ordinary structs smart pointers implement `Deref` and `Drop` traits. `Deref` handles how the dereferencing is handled; The `Drop` allows you to customize the code that runs when the instance is cleaned up.

Rust has a variety of smart pointers like -

- `Box<T>`: For allocating values on the heap.
- `Rc<T>`: For reference counted type that enables multi ownership.
- `Ref<T>` and `RefMut<T>` referenced through `RefCell<T>` which enforces rules of borrowing at run-time rather than at compile time.


## Box, Deref, Drop

### Using Box<T> to Point to Data on the Heap

Read from [./box/README_box.md](./box/README_box.md)

### Using Deref Trait to override `*` operator on a type

Read from [./box/README_deref.md](./box/README_deref.md)

### Drop Trait

The second trait important to the smart pointer is the `Drop`, which lets you customize what happens when the instance goes out of scope.

## Rc<T>, the reference counted smart pointer

In majority of cases ownership is clear and a single value is owned by a single owner. However there are certain cases a single value is owned by multiple owners. For example the graph data structures multiple edges might point to the same node and owned by multiple edges.

Read from [./rc/README_rc.md](./rc/README_rc.md)

## RefCell<T> and the Interior Mutability Pattern

Interior Mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; this action is disallowed by the borrowing rules. To mutate data the pattern uses `unsafe` code inside the data structures to bend Rust's usual rules. We can use types that use interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime even though the compiler can't guarantee that.



| Smart Pointer Type | Single vs. Multiple Owners | Mutable vs Immutable Borows | Compile Time vs Runtime | 
|--------------------|----------------------------|-----------------------------|-------------------------|
| Box<T>             | Single Owner               | Immutable & Mutable Borrows | Compile Time            |
| Rc<T>              | Multiple Owners            | Immutable                   | Compile Time            |
| RefCell<T>         | Single Owner               | Mutable Borrows Checked at runtime & Interior Mutability | Runtime                 |

