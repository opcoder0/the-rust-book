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

_NOTE_ - Here we are telling the compiler that the parameters' lifetime and that it should reject if the lifetime does not match.

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

*NOTE* What this says is the member `data` (`&'a str`) must live until the type `ImportantExpert` lives.

## Lifetime Elision

We learnt that every reference has a lifetime and the annotation needs to be specified for the function / struct. However in [Chapter 4](../ch4/slice_type/src/main.rs:73) we did not specify a lifetime and the code worked.

```
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

We don't have to do this -

```
fn first_word_slice<'a>(s: &'a str) -> &'a str
```

Prior to Rust 1.0 every reference required specifying lifetime annotations. Rust now recognizes common patterns in developers code from which it is able to infer the lifetimes. Going further more such cases/patterns might be identified and there would be even lesser cases to specify the annotations.

These patterns programmed into Rust's reference analysis is called _lifetime elision rules_. These rules are not for programmers but for the Rust compiler. When rust applies these rules and still finds ambiguity then it issues an error to the user asking the user to specify lifetime annotations.

_Lifetimes_ on functions / method parameters are called _input lifetimes_
_Lifetimes_ on return values are called _output lifetimes_

**There are 3 Lifetime Elision Rules -**

The compiler uses these three rules to figure out the annotations when they are not specified explicitly.

- *RULE-1* 
  - This applies to the input lifetimes.
  - The compiler assigns a lifetime parameter to each parameter that is a reference. As an example -

  A function with one parameter -
  ```
    fn foo<'a>(x: &'a str)
  ```

  A function with two parameters gets two separate lifetime parameters -
  ```
    fn foo<'a, 'b>(x: &'a str, y: &'b str)
  ```

- *RULE-2*
  - If there is exactly one input lifetime parameter then that lifetime is assigned to all the output lifetime parameters. Example -
  ```
    fn foo<'a>(x: i32, s: &'a str) -> &'a str
  ```

- *RULE-3*
  - If there are multiple input lifetime parameters but one of them is `&self` or `&mut self` (methods) the lifetime of `self` is assigned to all output parameters.


Let's pretend to be the compiler and apply the lifetime parameters on the following signtures -

| Statement                                | Rule - 1                                               | Rule - 2                                     | Rule - 3 | NOTES                                                                                              |
|------------------------------------------|--------------------------------------------------------|----------------------------------------------|----------|----------------------------------------------------------------------------------------------------|
| `fn first_word(s: &str) -> &str {`       | `fn first_word<'a>(s: &'a str) -> &str {`              | `fn first_word<'a>(s: &'a str) -> &'a str {` | N/A      |                                                                                                    |
| `fn longest(x: &str, y: &str) -> &str {` | `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {` |                                              |          | Compiler throws an error as it cannot determine if the output lifetime needs to come from 'a or 'b |


## Lifetime annotations in method definitions

When the struct with references have `impl` block the lifetime annotations needs to be specified there too. 

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn version(&self) -> i32 {
        1
    }

    fn announce_and_return_part(&self, announce: &str) -> &str {
        println!("Annoucement: {}", announce);
        self.part
    }
}
```

For the function `announce_and_return_part` the compiler -

- Applies Rule-1. It assigns a lifetime for each reference. One for `&self` and one for `&str`.
- Rule-2 is not applicable as there are more than one parameter.
- Applies Rule-3: The lifetime of `self` is assigned to the return value.

## The Static Lifetime

`'static` is a special lifetime. This is used to indicate that the reference is valid for the entire life of the program. Example -

```
let s: &'static str = "This is a static string";
```

## Generic Trait Parameters, Trait Bounds, Lifetimes


A function with generic trait parameters, trait bounds and lifetimes all together -


```
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("announcement: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
