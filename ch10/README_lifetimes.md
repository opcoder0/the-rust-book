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


