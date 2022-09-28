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
