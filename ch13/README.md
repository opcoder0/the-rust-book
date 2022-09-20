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
