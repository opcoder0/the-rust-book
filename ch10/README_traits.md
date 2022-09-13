## Traits

Traits define shared behavior in an abstract way. They are similar to interfaces in other languages. But has some differences.

## Defining a Trait

As an example let's define two structs `NewsArticle` and `Tweet`. Each of these hold data in them. We can define a `Summary` trait that these structs can implement -

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a Trait on a Type

Defining the two structs `NewsArticle` and `Tweet` and implement the `Summary` trait on the types -

```
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
	    format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
	    format!("{}: {}", self.username, self.content)
	}
}
```

_NOTE_ To invoke the methods of the trait on a type both the trait and type needs to be brought into scope. Example - `use aggregator::{Summary, Tweet};`

## Trait Implementation Rules

A type can implement a trait only if atleast one of the trait or the type is local to our crate. From the above example we can implement the `Summary` trait on `Vec<T>` or implement `Display` trait on `Tweet`. But we cannot implement `Display` trait on `Vec<T>` as both the trait and type are out of our crate. This property is known as _coherence_ (more specifically _the orphan rule_). 

## Default Implementations

Sometimes it is useful to define a default implementation for a trait. Example -

```
pub trait Summary {
	fn summarize(&self) -> String {
	    format!("(Read more...)")
	}
}
```

To use the default implementation on a type -

```
impl Summary for NewsArticle {}
```

_NOTE_ - `impl` has an empty body for the type.

```
fn main() {
	let t = Tweet {
		username: String::from("tcm"),
		content: "era of abundance... is it really?",
		retweet: false,
		reply: false,
	}
	println!("{}", t.summarize());
}
```

The above code will call the default implementation.
