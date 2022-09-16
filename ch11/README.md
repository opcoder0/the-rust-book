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

