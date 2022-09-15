## Tests

- A function that is annotated with `test` i.e. `#[test]` is considered a test and is executed when `cargo test` is run.
- You can have as many test modules and functions as needed.
- Test fails when something in the test function panics.
- Each test is run in its own thread. When the main thread sees a particular thread died the test is marked as failed.
- The `assert!` macro is useful for checking if a condition passed in tests. For a `true` value the `assert!` macro does nothing. If the value is `false` the macro panics.



