## Option Enum

Rust does not have the "null" feature. It instead uses `Option` enum that is defined in the standard library. It encodes the very common scenario when the value could be _something_ or _nothing_. 

```
enum Option<T> {
    None,
    Some(T),
}
```

The `Option` enum is so useful that it is included in the prelude. `Option`, `Some` and `None` can all be used as-is in the code without imports. 

```
    let some_number = Some(20);
    let no_number = None;
```

Here -

- `some_number` is of type `Option<i32>` and
- `no_number` is also of the same type `Option<i32>`. The compiler cannot determine the type for this one by looking at `None` as opposed to `some_number` which knows `Some(20)` where T is `i32`.

But the code that distinguishes the value would know if the value is valid if the value of the enum is `Some(T)` as against `None`.

```
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    // --> src/main.rs:7:17
    //   |
    // 7 |     let sum = x + y;
    //   |                 ^ no implementation for `i8 + Option<i8>`

```

`T` and `Option<T>` are different.

## Getting `T` out of `Option<T>`

### Using match control flow

```
    // case-1
    let n = Some(5);
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);

    // case-2
    let n: Option<i32> = None;
    let m = plus_one(n);
    println!("plus_one({:?}) => {:?}", n, m);

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}
```

For the case-1 `x` matches `Some(n)` arm and `n` holds 5;
For the case-2 `x` matches `None` arm and returns back `None`;

### Using `if let` syntax

For cases where `match` is wordy `if let` can be used. For some cases using `match` is verbose and has redundant code. `if let` provides a much cleaner syntax -

```
let optional = Some(7);
match optional {
    Some(v) => println!("Found value: {}", v),
    _ => {},
}
```

The above case shows a no-op for the None case. This could be expressed using `if let` as -

```
let optional = Some(7);
if let Some(v) = optional {
    println!("Found value: {}", v);
} 
// else can be specified if required.
```
