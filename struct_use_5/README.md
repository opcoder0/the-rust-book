## Struct Usage

To print a struct it needs to implement the `std::fmt::Display` and `std::fmt::Debug` trait. This gives the user control over how/what to print from the struct. To use the default/dev friendly output of all members use the `{:?}` or `{:#?}` format specifiers after the struct specifies the outer attribute of `#[derive(Debug)]`.

To get rid of the error add the `#[derive(Debug)]` attribute.

```
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

...
...

    let mut r = Rectangle {
        height: 36,
        width: 24,
    };
   println!("{:#?}", r);
```

## Debug Macro

dbg! macro can be used to print values to stderr. The macro takes ownership and returns it back. In order to receive a value back into the variable it needs to be mutable.

```
fn main() {
    let mut r = Rectangle {
        height: 36,
        width: 24,
    };
    // dbg! macro can be used to print values to stderr
    // The macro takes ownership and returns it back.
    r = dbg!(r);
    //    println!("{:#?}", r);
    dbg!("Area: {}", area(&r));
}

```
 
