## Struct Implementation

Method to struct -

- added in the `impl` block
- always takes `self` as its first argument

[Here](./src/main.rs), the method `area` takes &self as its first argument. Here `&self` is shortcut to `&self: Self`.

A method can -

- Borrow a non-mutable reference of self (`&self`)
- Borrow a mutable reference to self (`&mut self`)
- Take ownership of self (`self`)

## Method call

Rust methods can be invoked using the syntax `type_instance.method_name`. Unlike C/C++ it does not have the `->` operator. Rust uses _automatic referencing and dereferencing_. It uses the `self` argument on the method to determine the correct calling syntax.

## Associated Functions

Functions in the `impl` block without the first parameter as `self` is an associated function. Invoked by `type::method_name` syntax. It can use `Self` alias in it.

## Multiple impl blocks

Each struct is allowed to have multiple `impl` blocks.
