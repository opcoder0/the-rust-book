## Collections

Template based collections are defined in the standard library (`std::collections`); The data here is stored on the heap and hence the size of the data can be grown / shrunk at runtime. Some of the most common collections are `Vec` available in `std::vec` which allows you to store data contiguously and `String` which is a sequence of characters and `HashMap` a key-value pair accessed by hashing. There are others like `VecDequeue`, `LinkedList`, `BTreeMap`, `HashSet` etc (in `std::collections`).

## Vectors

Vectors (`std::vec`) -

- Is a generic container type
- Store elements contiguously on the heap
- All elements must be of the same type
- When a vector is dropped all its elements are dropped as well

_NOTE_: To store different element types in a vector use enum with associated values.

## Strings

Rust only has only one representation of string in the core language, called `str` (string slice). Usually referred via borrowed reference (`&str`). String literals are stored in the program binary. A `String` is a wrapper over `Vec<u8>`.



The `String` type (`std::string`) is -

- UTF-8 encoded sequence of bytes
- Growable, mutable, stored on the heap
- Implemented as a vector of bytes. So all operations on vector work on string too
- Indexing into a string is tricky when dealing with multibyte UTF-8 encoded text is stored.

Rust uses both `String` and `&str` in the standard library.

Concatenating string can be done by -

- `.push_str` function
- `+` operator
- `format!` macro

### The + operator

The `+` operator calls the `add` method internally which has the signature -

```
    // concat
    let s1: String = String::from("For rust internals");
    let s2: String = String::from(" read the book");
    // below s1 is moved so cant be used after this line
    let s3: String = s1 + &s2;
    println!("{}", s3);
```

```
fn add(self, s: &str) -> String 
```

The function takes ownership of `self` in this case `s1` and `s2` accepts a reference. Rust uses _defer coercion_ which turns `&s2` to `&s2[..]`.

We can see in the signature that `add` takes ownership of `self`, because `self` does not have an `&`. This means `s1` will be moved into the `add` call and will no longer be valid after that. So although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.


### The format! macro

The `format!` macro is like `println!`. Except that it returns a `String` back.

### Indexing String

Rust does not support indexing into string. The code below does not compile -

```
fn main() {
  let s: String = String::from("hello");
  let h = s[1]; 
}
```

Indexing into `&str` -

```
  let s = "hello";
  let h = &s[0..1];
  println!("s: {}, h: {}", s, h);
```

*NOTE* Rust panics if the indexing does not fit the grapheme cluster.
