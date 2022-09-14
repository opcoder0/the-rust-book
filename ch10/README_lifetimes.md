## Lifetimes and References

In Rust every reference has a lifetime, which is the scope the reference is valid. Most of the lifetimes are implicit and inferred. Just as we annotate types when more than one type is possible; we annotate lifetime _only_ when lifetimes of references could be related in a few different ways. Rust uses the _generics_ syntax to annotate lifetimes. 
## Preventing dangling references with lifetimes

The main aim of _lifetimes_ is to prevent _dangling references_. The example below creates a dangling reference -

```
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```

The compiler catches this error -

```
 --> src/main.rs:5:13
  |
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {}", r);
  |                       - borrow later used here

```

Rust uses the borrow checker to catch such errors.

## The Borrow Checker

The Rust compiler has a borrow checker that compares the scopes to determine if all borrows are valid.

```
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

The compiler sees that the lifetime of `r` which is `'a` is greater than lifetime of `x` which is `'b'`. And `r` refers to `x` which has lesser lifetime. The compiler catches this error as shown above.


## Generic Lifetimes in Functions

Here we'll write a function (`longest`) that takes two string slices and returns the longer one. 

```
fn main() {
    let s1 = String::from("abcd");
    let s2 = "this is a long string";

    let result = longest(s1.as_str(), s2);
    println!("longer string: {}", result);
}

fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

The compiler gives out an error -

```
--> src/main.rs:9:35
  |
9 | fn longest(s1: &str, s2: &str) -> &str {
  |                ----      ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  |           ++++      ++           ++          ++

```

## Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references live. Rather they describe the relationships of the lifetimes of different references. A lifetime annotation is represented by an apostrophe (') followed by a short name or a character. Example `'a` or `'b` etc.

Syntax -

```
&i32        // a reference
&'a i32     // a reference with explicit lifetime annotation
&'a mut i32 // a mutable reference with an explicit lifetime annotation
```

A single lifetime annotation by itself does not have any meaning as they are meant to describe the lifetime relationship between references.

## Lifetime annotations in function signatures

Now to fix the above error with the `longest` function -

```
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

This tells the compiler -

- the function accepts references to two string slices and they will both live atleast as long as lifetime `'a`. 
- the return reference also lives atleast as long as the parameters.

_NOTE_ - Here we are telling the compiler that the parameters' lifetime and that it should reject any other lifetimes.

## Testing annotations by passing references with different lifetimes

Now let's check what happens if we pass references with different lifetimes to the above example (`longest`). Example -

```
fn main() {
    let s1 = String::from("this is a long string");
    let result;
    {
        let s2 = String::from("abcd");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("longer string is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Gives the following error -

```
error[E0597]: `s2` does not live long enough
 --> src/main.rs:6:39
  |
6 |         result = longest(s1.as_str(), s2.as_str());
  |                                       ^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `s2` dropped here while still borrowed
8 |     println!("longer string is: {}", result);
  |                                      ------ borrow later used here
```

## Thinking in terms of lifetimes

How do we specify a lifetime annotation ?

Lifetime annotation is required and describes the lifetime of a reference with respect to another value (either another function parameter or return value). A single lifetime annotation parameter in itself is of no use.

## Lifetime annotations in struct definitions

If struct members are references then lifetime annotation needs to be added to every reference. Example -

```
struct ImportantExpert<'a> {
    data: &'a str,
}

struct LinkedList<'a> {
    data: i32,
    next: &'a LinkedList<'a>,
}
```
