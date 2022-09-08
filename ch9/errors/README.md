## Error Handling

Rust group error handling into two kinds -

- Recoverable errors
- Unrecoverable errors 

Rust has no exception based error handling. In many cases issues compilation errors when errors are not handled.

### Recoverable errors

Errors that need to be handled. Rust returns `Result<T, E>`.


### Unrecoverable errors

Unrecoverable errors are handled by stopping the program. This can be implemented by `panic!` macro. When `panic!` is called Rust -

- prints a message
- clean up
- unwind the stack
- exit 

With an environment variable `RUST_BACKTRACE=1` the program prints the stack frames that were unwound. 

_NOTE_ - Unwinding the stack and clean up of resources is lot of work. To keep the executable small and quickly exit the `panic!` call could instead abort. This is declared in `Cargo.toml` as -

```
[profile.release]
panic = 'abort'
```

```
fn main() {
    println!("Hello, world!");
    panic();
}

fn panic() {
    panic!("panicccc!");
}
```

**Debug Output**

```
Hello, world!
thread 'main' panicked at 'panicccc!', src/main.rs:7:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

With `RUST_BACKTRACE=1`

```
Hello, world!
thread 'main' panicked at 'panicccc!', src/main.rs:7:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:142:14
   2: errors::panic
             at ./src/main.rs:7:5
   3: errors::main
             at ./src/main.rs:3:5
   4: core::ops::function::FnOnce::call_once
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```

**Release Output (with panic = abort) setting**

```
Hello, world!
thread 'main' panicked at 'panicccc!', src/main.rs:7:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Aborted (core dumped)
```

With `RUST_BACKTRACE=1`

```
Hello, world!
thread 'main' panicked at 'panicccc!', src/main.rs:7:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:142:14
   2: errors::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Aborted (core dumped)
```
