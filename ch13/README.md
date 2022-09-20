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
