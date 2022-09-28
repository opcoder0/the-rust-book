# More about cargo and crates.io

This chapter covers -

- Customize your build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend cargo using custom commands

## Customize your build through release profiles

Cargo has 4 builtin profiles -

- dev (`cargo build`, `cargo run`)
- release (`cargo build --release`, `cargo run`)
- test (`cargo test`)
- bench (`cargo bench`)

Cargo has good defaults for each of these profiles when building. To override a setting add the section `[profile.release]` or `[profile.dev]` in `Cargo.toml` file and specify the override setting. For a full list of cargo profile options [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html).

## Publishing a crate to crates.io

Rust and cargo have features to publish packages to crates.io

### Making useful documentation

Package documentation can be provided using the `///` comment syntax. The content in the `///` comment supports markdown sytax. `cargo doc` generates the documentation into `target/doc` which can be published to crates.io. Example -

```
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

### Documentation comments as tests

The test code as shown above is run when running `cargo test` under doc-tests section. This allows the code owner to make sure the documentation and code is always up-to-date.

### Commenting contained items

The comment style `//!` can be used to document the item that is contained rather than the item that follows. Example `//!` can be used to document **modules / entire crates**. Example -

```
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convinient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```


