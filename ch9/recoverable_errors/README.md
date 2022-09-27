## Recoverable Errors

Handling failures with `Result` type. The `Result` enum has two variants -

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Ok(T)` returns the result of the operation.
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


## Propogating Errors

Propogating error to the caller. Instead of handling errors in the function a `Result<T, E>` is returned -

```
fn read_username() -> Result<String, io::Error> {
    let result = File::open("username.txt");
    let mut username_file = match result {
        Ok(f) => f,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    let result = username_file.read_to_string(&mut username);
    match result {
        Ok(_) => return Ok(username),
        Err(err) => Err(err),
    }
}
```

Propogating errors is a very common scenario. So there is a shortcut for this in Rust.

## Propogating Errors: A shortcut using ? operator

The `?` operator placed after a `Result` value works the same way as the `match` operator defined above. If the `Result` returned was 
- `Ok` value then the contained value returned to the expression
- `Err` the error is returned from the function.

```
fn read_username_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

## Where can the ? operator be used

The `?` operator can be used in functions whose return type is compatible with the value the `?` is used on.

Example -

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

The code above does not compile because the `?` operator follows the `Result<T, E>` value returned by `File::open` while `main` returns `()` is not compatible with `Result<T, E>`.

The `?` operator can be used with values of type -

- `Result<T, E>`
- `Option<T>`
- Values that implement `FromResidual<Result<Infalliable, std::io::Error>>`

The `main` function returns `()` (by default when it doesn't return anything). `main` being special can only return certain types/values. The code can be changed to -

```
fn main() -> Result<(), Box<dyn Error>> {
  let f = std::fs::File::open("hello.txt")?;
  Ok(())
}
```

This function above can return `()` or `Box<dyn Error>` - which means any type of error. 

*NOTE* - The `main` function can return any types that implement the trait `std::process::Termination`.
