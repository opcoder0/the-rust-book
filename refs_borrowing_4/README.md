## Mutable References

```
let mut s1 = String::from("hello");
let r1 = &mut s;
```

`r1` is a mutable reference to `s1`.

## Restrictions or Rules

- If you have a mutable reference to a value, you can have no other references to that value.
- Rust enforices a similar rule for combining mutable and immutable references. Which is - Cannot have a mutable reference while there are immutable references to the same value (in the same scope).
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

_NOTE_ The ability of the compiler to tell that a reference is no longer being used at a point before the end of scope is called [Non-Lexical Lifetimes](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html) (NLL for short).

## Dangling References

Rust prevents danging references.

Rust prevents this by issuing a compiler error -

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```
