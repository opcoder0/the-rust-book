## Organize Rust Code

Rust has number of features to allow you to organize code -

- *Packages* - A Cargo feature that lets you build, test and share crates.
- *Crates* - A tree of modules that produce a library or executable.
- *Modules* and *use* - Let you control the organization, scope, and privacy of paths
- *Paths* - A way of naming an item such as struct, function, or module.

**Crate** 

Smallest piece of code the rust compiler `rustc` or `cargo` considers at a time. _Crate_ can contain modules. Two types of crates -

- Library crate
- Binary crate

**Crate Root** 

Crate root is the source file the compiler starts from to make up the root module of your crate. Crate roots -

- Library crate (`src/lib.rs`)
- Binary crate (`src/main.rs`)

**Package** 

A package is a bundle of one or more crates. It can contain multiple binary crates but only one library crate. Running `cargo new` generates -

```
.
├── Cargo.toml
└── src
    └── main.rs
```

Cargo follows the convention -

- `src/main.rs` is the crate root of a binary crate with the same name as the package
- `src/lib.rs` is the crate root of the library crate with the same name as the package

A Package can have multiple binary crates by placing the files in `src/bin` directory. Each file is a binary crate -

```
.
├── Cargo.toml
└── src
    └── bin
        └── list.rs
        └── append.rs
        └── delete.rs
    └── main.rs
```

Cargo passes the crate root to the compiler.

**Module**

In the crate root file you can declare modules. You declare modules using the following syntax `mod modname;`. Example -

```
// in src/main.rs
mod garden;
```

The compiler will look for module's code in -

- inline in the source for code like `mod garden {}`
- in the file `src/garden.rs`
- in the file `src/garden/mod.rs`

**Sub-modules**

Sub-modules can be declared in any other file other than crate root. For example you may declare -

```
// in src/garden.rs
mod vegetables;
```

The compiler will look for the submodule code within the directory named for the parent module in these places -

- inline in `src/garden.rs` directly following `mod vegetables;`
- in the file `src/garden/vegetables.rs`
- in in the file `src/garden/vegetables/mod.rs`

**Paths**

Once the module is part of your crate you can refer to the module from anywhere within the same crate as long as the privacy rules are followed. For example `Asparagus` in the garden vegetables module would be found `crate::garden::vegetables::Asparagus`.

**Private vs Public**

**NOTE:** The items defined in child modules (functions, methods, struct, enums, modules, constants) are all private to the parent modules.

Code within the module is private from it's parent modules. Use `pub` (`pub mod|fn|struct|enum|<type>`) to make it public.

*Use*

The `use` keyword creates shortcuts to items to reduce the path length. Writing `use crate::garden::vegetables::Asparagus` let's us use `Asparagus` directly in the code. Paths can take two forms -

- Absolute path: 
  - To refer modules in the same crate the absolute path starts with `crate::`.
  - To refer to modules from an external crate the absolute path starts with the package name.
- Relative path:
  - It starts from the current module and uses `self` or `super` or refers to something in the current module.

In Rust bringing the parent module into scope and then calling a function or referring to a type is considered idomatic. This gives more clarity. It affirms that it belongs to a module rather than defined locally. On the contrary for `struct|enum` it is idomatic to bring the type into scope and use it -

```
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```

When bringing types into scope Rust does not allow using the types with same name without resolution. Example -

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    println!("in function1");
    Ok(())
	
}

fn function2() -> io::Result<()> {
    println!("in function2");
    Ok(())
	
}

fn main() {
   _ = function1();
   _ = function2();
  
}
```

**New names with `as` keyword**

The `use` keyword can be used with `as` keyword to import a type under a different name. Example -

```
use std::fmt::Result;
use std::io::Result as IoResult;
```

**Re-exporting names with `pub use`**

A module that brings in another name into scope can re-export it under a different name using `pub use` keywords. Example -

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // re-exported as public hosting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Before this change, external code would have to call the `add_to_waitlist` function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`. Now that this pub use has re-exported the hosting module from the root module, external code can now use the path `restaurant::hosting::add_to_waitlist()` instead.

**Using Nested Paths to clean up long `use` lists**

```
use std::cmp::Ordering;
use std::io;
```

can be re-written as -

```
use std::{cmp::Ordering, io};
```

OR

```
std::io;
std::io::Write;
```

can be rewritten as -

```
std::io{self, Write};
```

**The Glob Operator**

If you want to bring all public members from a module then use the `*` glob operator. Example -

```
use std::collections::*;
```
