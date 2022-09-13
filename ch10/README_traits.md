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
