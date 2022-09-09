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
