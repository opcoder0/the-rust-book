## Recoverable Errors

Handling failures with `Result` type. The `Result` enum has two variants -

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Ok(T) returns the result of the operation.
`Err(E)` returns the error in case the operation failed. 

This can be matched using `match` arms -

```
let result = std::fs::File::open("greeting.txt");
let handle = match result {
    Ok(f) => f,
    Err(err) => panic!("cannot open file greeting.txt {}", err),
};
```

## Recoverable Errors: Matching on different errors

```
let result = std::fs::File::open("greeting.txt");
let handle = match result {
    Ok(f) => f,
    Err(err) => panic!("cannot open file greeting.txt {}", err),
};
```

To the above program to add logic to create the file when file was not found but panic on other errors -

```
    let result = File::open("greeting.txt");
    let handle = match result {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(f) => f,
                Err(err) => panic!("Cannot create file: {}", err),
            },
            other_error => panic!("Cannot open file: {}", other_error),
        },
    };
```

Alternative methods to implement the above logic using closures that uses concise syntax. More details in chapter 13.

```
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
```

## Shortcuts for panic on error: unwrap and expect

For cases where code needs to 'panic on error' there are shortcut methods on the `Result<T, E>` type -

- `unwrap`
- `expect`

### Unwrap

`unwrap` returns the contained value and consumes the self. In case of an error `unwrap` panics with the error.

### Expect

`expect` returns the contained value and consumes the self. In case of an error `expect` panics with an additional error message adding context.
