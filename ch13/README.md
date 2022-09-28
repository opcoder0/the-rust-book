## Functional language features: Iterators and Closures

Programming in functional style often includes -

- Using functions as values by passing them in arguments
- Returning them from other functions
- Assigning them to variables for later execution 

and so on. 

## Topics covered

- _Closures_, a function-like construct you can store in a variable.
- _Iterators_, a way of processing a series of elements
- How to use Closures and Iterators to improve the I/O project (in Chapter 12)
- The performance of Closures and Iterators.

Mastering Closures and Iterators are part of writing idiomatic Rust code.

## Closures: Anonymous functions that capture their environment

In Rust Closures are anonymous functions you can save in a variable and pass to other functions. You can create a closure in one place and call it elsewhere. Unlike functions Closures capture values from the scope in which they are defined.

### Capturing the environment with Closures

Illustrating usage of closures with an example - A company issues T-shirts of a certain colour if it is available in their inventory otherwise issues which ever colour it has the most of. As shown here in [example: closure](./closure_tshirts/src/main.rs).

### Closure type inference and annotation

Closures typically don't require you to annotate types for parameters or the return value. This is because unlike regular functions which are exposed for others to invoke closures are usually small functions within a function and is valid only in that narrow context. As with variables closures also can be annotated. The examples below show type annotation and various ways a closure can be written -

```
    let expensive_function = |num: u32| {
        println!("performing expensive calculation");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = expensive_function(2);
    println!("expensive function: {}", x);

    // regular function
    fn add_one_v1(x: u32) -> u32 { x + 1 }

    // closure with type annotation
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // type assertion is set during the first call
    let add_one_v3 = |x| {x + 1};

    // remove the braces
    let add_one_v4 = |x| x + 1;
```

### Capturing References or Moving Ownership


A Closure can capture variables from its environment in 3 ways which directly map to the 3 ways a function receives an argument -

- Borrow immutably
- Borrow mutably
- Take ownership

The closure will decide which of these to use based on what the closure is doing.

#### Borrow Immutably

Example -

```
fn main() {
    let list = vec![1, 2, 3, 4];
    println!("Before defining closure: {:?}", list);
    let borrow_immutably = || println!("from closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    borrow_immutably();
    println!("After calling closure {:?}", list);
}
```

#### Borrow Mutably

Example -

```
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("Before declaring closure: {:?}", v1);
    let mut mutable_borrow = || v1.push(5);
    //
    // println!("Before calling closure {:?}", v1);
    // ^
    // |__  will result in error saying cannot borrow 
    //      immutably when it is already borrowed mutably
    //
    mutable_borrow();
    println!("After calling closure {:?}", v1);
}
```

#### Taking Ownership

If you want the closure to take ownership you can use the `move` keyword before the parameter list. This is useful when passing closure to a new thread. The data is moved to the thread so that it is owned by the thread. More on `move` closures in Chapter 16 when discussing concurrency.

### Moving Captured Values Out of Closure and the Fn Traits

Once a closure has captured a reference or captured ownership of a value where the closure is defined (thus affecting, if anything, is moved _into_ the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (thus affecting, if anything, what is moved _out of_ the closure).

A closure body can do any of the following - 

- Move a captured value out of the closure
- Mutate the captured value 
- Neither move nor mutate value 
- Capture nothing from the environment to begin with.

The way the closure captures and handles values from the environment affects which traits the closure implements and traits are how structs and functions can specify what kinds of closures they can use. Closures will automatically implement one, two or all three of these `Fn` traits in an additive fashion.

- `FnOnce` applies to closures that is called atleast once. All closures implement this trait because they are called atleast once. A closure that moves values outside its body implement `FnOnce` and none of the other `Fn` traits because it can be called only once.

- `FnMut` applies to closures that don't move values outside their body but mutate them inside. These can be called more than once.

- `Fn` applies to closures that don't move captured values outside the body. They don't mutate the values either. They also don't capture anything from their environment. Such closures can be called multiple times from multiple threads.


#### Unwrap_or_else

Let's look at an example of `unwrap_or_else` implementation on the `Option<T>` -

```
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self: f: F) -> T 
    where
    F: FnOnce() -> T {
	    match self {
	        Some(x) => x,
		None => f(),
	    }
    }
}
```

Note that the `unwrap_or_else` takes another generic argument of `F`. `F` is the closure. As declared in the `where` clause `F` is trait bound to `FnOnce`. Using the trait bound `FnOnce` in `unwrap_or_else` expresses the constraint that `unwrap_or_else` calls `f()` only once. 


#### sort_by_key

Now let's look at `sort_by_key` function. This function is used to sort a slice by a given key. [sort_by_key](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.sort_by_key) -

```
pub fn sort_by_key<K, F>(&mut self, f: F)
where
    F: FnMut(&T) -> K,
    K: Ord,
```

`sort_by_key` uses `FnMut` instead of `FnOnce` trait bound. Example -

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = vec![
        Rectangle {
            width: 2,
            height: 3,
        },
        Rectangle {
            width: 4,
            height: 5,
        },
        Rectangle {
            width: 1,
            height: 2,
        },
        Rectangle {
            width: 3,
            height: 4,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("Sorted by keys: ");
    println!("{:#?}", list);
}
```

The closure of `sort_by_key` gets one argument which is a reference to the element of the slice. The closure needs to return the key (`K`) which needs to implement `Ord` (i.e. could be used for ordering elements).

The next example shows how a `value` is used to keep count of how many times the key-fn (closure body) is called from `sort_by_key`. Refer to the [example here](./sort_by_key_count_calls/src/main.rs). The code here does not work. The code attempts to keep count by pushing the `value` in to the `counter` each time the closure is called. However this is not allowed. Because after the `value` is **_moved_** into `counter` the first time there is nothing left in the `value`. The compiler throws the error -


```
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:25:22
   |
22 |       let value = String::from("sort key function has been called");
   |           ----- captured outer variable
23 |       let mut counter: Vec<String> = Vec::new();
24 |       list.sort_by_key(|r| {
   |  ______________________-
25 | |         counter.push(value);
   | |                      ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
26 | |         r.width
27 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `sort_by_key_count_calls` due to previous error
```

It is instead easier to have a counter and increment the counter each time the key-fn is called. Shown here - [./sort_by_key_count_calls_incr/src/main.rs](./sort_by_key_count_calls_incr/src/main.rs).

## Processing a series of items with Iterators

Iterators let you iterate over each element in a collection. Example of an iterator on a vector -

```
let v1 = vec![1, 2, 3, 4];
let iter = v1.iter();
```

The above initializes the iterator to the vector `v1`. The code below loops the vector `v1` using the iterator in the for loop -

```
for val in iter {
    println!("val: {}", val);
}
```

Here the `for` loop takes ownership of `iter` and 

### Iterator trait and the next() method

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

**NOTE** The iterator defines an associated type (`Item`). More on associated types in Chapter 19. Here with the signature of the `next` method it says who ever implements `Iterator` trait must have a type of `Item`.

Note that the `next()` method returns each value (i.e. `Some(&val)`) in the collection and when it reaches the end it returns `None`. Code here -

```
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut iter = v1.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}
```

**NOTE** Note that the `for` loop [here](./iterator_basics/src/main.rs:9) does not need a mutable iterator because it takes ownership of the iterator and uses it as required internally.

The Iterator trait has number of methods defined with default implementations.

### Methods that consume the iterator 

Methods that call `next()` are said to consume the iterator. One such method is the `sum`. This calls `next` repeatedly and adds up the values of elements in the iterator.

```
fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4];
    let iter = v1.iter();
    let total: i32 = iter.sum();
    println!("Sum of vector {:#?} is {}", v1, total);
}
```

### Methods that produce other iterators (Iterator Adaptors)

Iterator adaptors are different methods defined on the iterator trait. They don't consume the iterator but instead produce iterators with different properties by changing some aspect of the original iterator.

Example -

```
fn main() {
    let v = vec![1, 2, 3, 4];
    let iter = v.iter().map(|x| x + 1);
    // calls the closure above for each element. 
    // The vector remains unchanged
    for elem in iter {
        println!("{}", elem);
    }
    println!("vector: {:?}", v);
}
```

## Improving the I/O project

Now with the new skills around using `iterator` and `closures` the `minigrep` code can be further improved / refactored. Changes -

- remove the `.clone()` in the `build` method and use an `iterator` instead of `Vector<String>` arguments.
- Refactor the loops in `search` and `search_case_sensitive` to use `filter` iterator adaptor instead of looping.

[Updated code is here.](../ch12/minigre/lib.rs)

Iterator is the Rust idimomatic approach most programmers take. As it is faster and avoids erroneous bits of looping (like erroneous end conditions etc.).

## Comparing Performance: Loops vs Iterator

Iterators provide high level abstractions for looping which have the same / better performance than using loops.
