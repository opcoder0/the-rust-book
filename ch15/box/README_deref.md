# Deref Trait

## Treating a type like a reference by implementing Deref trait

In order for a type to be treated as a reference the `Deref` trait needs to be implemented for the type.

The `Deref` trait defines one method `deref` which borrows `self` and 
returns a reference to the inner data. So in our case for `MyBox` that would be

```
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
    	&self.0
    }
}
```

Without the `Deref` trait the type cannot be dereferenced. 

**NOTE** Behind the scenes the Rust compiler changes `*y` to `*(y.deref())`

## Implicit Deref Coercion

Deref coercion converts a reference type that implements the Deref trait into a reference of another type. For example the `String` type implements the Deref coercion that converts `&String` to `&str`. The example [here](./deref_coercion/src/main.rs) shows that the call to `decorate_print` does this -

- MyBox<String>::deref returns &String
- Rust compiler calls &String::deref which returns &str and that is passed to `decorate_print`

## How Deref Coercion acts on Mutability

Just like you use `Deref` trait to override the `*` operator on an immutable type, you can use `DerefMut` type to override `*` on a mutable type.

Rust does defer coercion when it finds types and trait implementation in these three cases -

- `T` to `U` when: `T: Deref<Target=U>`
- `&mut T` to `&mut U` when: `T: DerefMut<Target=U>`
- `&mut T` to `&U` when `T: Deref<Target=U>`
