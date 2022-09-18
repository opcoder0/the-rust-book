## Tests

## Basics

- A function that is annotated with `test` i.e. `#[test]` is considered a test and is executed when `cargo test` is run.
- You can have as many test modules and functions as needed.
- Test fails when something in the test function panics.
- Each test is run in its own thread. When the main thread sees a particular thread died the test is marked as failed.
- The `assert!` macro is useful for checking if a condition passed in tests. For a `true` value the `assert!` macro does nothing. If the value is `false` the macro panics.
- The `assert_eq!` and `assert_ne!` macros can be used to compare two values.
- `assert`, `assert_eq` and `assert_ne` macros can be passed custom messages after the actual arguments. i.e.
   - `assert!(<condition>, <message>)`
   - `assert_eq!(<left>, <right>, <message>)`
   - `assert_ne!(<left>, <right>, <message>)`
   - The `<message>` is sent to the `format!` macro.


## Checking panics with `should_panic`

Check panics with `should_panic` annotation `#[should_panic]`. To check conditions where a particular function should panic for certain cases.

Some functions might panic for reasons the test is not looking for. Even under such cases the `#[should_panic]` records it as a panic and passes the test.

To catch such cases / specific panic cases `should_panic` can be used with `expected` parameter. The value here needs to be a substring of the expected panic message. Example -

```
pub struct Guess {
    n: i32,
}

impl Guess {
    pub fn new(i: i32) -> Self {
        if i < 1 {
            panic!("value should be more than 1");
        } else if i > 100 {
            panic!("value should be less than 100");
        } else {
            Guess { n: i }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "value cannot be zero")]
    fn should_panic_wrong_expected_message() {
        let _g = Guess::new(-1);
    }
}
```

The expected output from rust -

```
---- tests::should_panic_wrong_expected_message stdout ----
thread 'tests::should_panic_wrong_expected_message' panicked at 'value should be more than 1', src/lib.rs:8:13
note: panic did not contain expected string
      panic message: `"value should be more than 1"`,
 expected substring: `"value cannot be zero"`
```

The test function below catches the correct panicked case -

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "should be less than 100")]
    fn should_panic_more_than_upperbound() {
        let _g = Guess::new(200);
    }
}
```

## Using Result<T, E> in tests

So far we have seen functions that only panic. We could also have tests return a `Result<T, E>` which can later be handled with a `?` operator.


## Controlling How tests are run

Just like `cargo run` compiles and runs the code. `cargo test` compiles the code with a test binary and runs the tests. By default each test is run in a separate thread. This improves performance. 

It is important to note that each test should be independent of other tests. If there are any kind of dependencies like shared files, environment variables the tests could report failure due to race conditions. To resolve this the tests could be run in a single thread using `cargo test -- --test-threads=1` or remove dependencies between tests.

## Showing function output

By default, if a test passes, the Rust library captures anything that is printed to stdout. If the test fails the output is then shown along with the test output. To always display stdout output from test functions along with the test result run `cargo test --show-output`.

## Running subset of tests by name

To run a subset of tests by name pass the filter argument to `cargo test` example - `cargo test add` will run all the tests whose name has `add`. Give the full name of the test to run just one test.

## Ignoring some tests unless specifically requested

Some tests can be ignored using `#[ignore]` annotation. Example 

```
#[test]
#[ignore]
fn expensive_test() { }
```

The above test will be ignored by `cargo test`. 

To run only the ignored tests run `cargo test -- --ignored`. 
To run all the tests including the ignored ones run `cargo test --include-ignored`

## Test Organization

Rust community think of tests as two main types -

- unit tests - tests private functions from within the module.
- integration tests - tests the module from outside as a user of the library.

## Unit Tests

- Unit tests are placed in the `src` directory in the source files.
- By convention create a module named `tests` in each file and annotate it with `#[cfg(test)]`. This annotation tells Rust to compile and run this code only on `cargo test`. 

_NOTE_ Integration tests go in a different directory and don't need to be annotated with `#[cfg(test)]`.

## Testing private functions

All functions can be accessed from the tests module. So private/internal functions can be tested.

## Integration Tests

These tests are entirely external to the library. They invoke only public functions (APIs). 

- Create a directory called `tests` in the top level (same as `src`). Cargo knows to look for integration tests here. 
- Any number of integration test files can be placed in here.
- Each integration test file is a separate crate and the modules from the package needs to be imported into each test separately.
- There are 3 sections in the report after running `cargo test`. A section for 
  - Unit tests
  - Integration tests
  - Doc tests
  _NOTE_ If a test fails in a section following tests won't be run. For example if a test fails in the unit test the integration test and doc tests won't be run.

_NOTE-1_ We don't need to annotate integration tests with `#[cfg(test)]` as Cargo treats these as tests and compiles them only when running `cargo test`.
_NOTE-2_ Each integration test file has its own section. With each test function per line.

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── add_tests.rs
```

Running `cargo test` runs both the unit tests and integration tests.

## Submodules in integration tests

As mentioned each file in `tests` directory is considered a separate crate. It is helpful to organize integration test crates in a modular fashion and have helper functions etc. 

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── add_tests.rs
    └── common.rs     // <- common setup / helper functions used by other tests
```

*NOTE* `common.rs` is a separate crate. The tests will attempt run tests in `common.rs` which won't be found. To avoid running tests in crates with zero tests (like `common.rs`) a submodule can be created using the below convention

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

The above case will let the `common` module be used as a common function.

**NOTE** Only library crates can have integration tests not binary crates.
