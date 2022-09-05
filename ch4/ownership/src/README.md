# Ownership Rules

- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope the value will be dropped.

## Example

```
...
  {
    let s = "hello"
  }
  // scope of s is over and is not valid from here.
...
```

```
// The values here are copied.
let x = 5;
let y = x;

// for complex types
let s1 = String::from("hello");
let s2 = s1;
```
