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

**NOTE-1** The default implementation of a trait method can call other methods of the trait though they have not been implmented. This can provide useful functionality in the default implementation.

**NOTE-2** It is not possible to call the default implementation from an overridden implementation.

## Traits as parameters

Any function can accept a trait as a parameter. This allows the caller to pass in any of the types that implement the trait as arguments. To specify the parameter type use the syntax `impl <trait>`. 

This example adds `notify` function which takes a `Summary` trait as argument -

```
fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}
```

In the caller code -

```
notify(&tweet);
notify(&news);
```

## Trait bound syntax

The above method of using `impl <trait>` is a syntactic sugar for the longer notation of using trait bound sytax which uses generics to specify the `trait` as a generic type. Example -

```
fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}
```

The trait as parameter using `impl <trait>` syntax is good for specifying simpler cases. For more complex cases the trait bound syntax is suitable.

## Specifying multiple trait bounds using + syntax

More than one trait bound can be specified. 

Example -

```
fn notify(item: &(impl Summary + Display))
```

Same using the generic type notation -

```
fn notify<T: Summary + Display>(item: &T)
```

## Clearer trait bounds with `where` clause

Having multiple trait bounds leaves the above syntax cluttered and difficult to read/write. The `where` clause is used in such cases

```
fn to<T, U>(t: &T, u: &U) -> T 
	where T: Display + Clone,
	      U: Diplay + Debug
{

}
```

## Returning types that implement Traits

The `impl <trait>` syntax can also be used in the return position to return anything that implements a particular trait. Example -

```
fn return_summarizable() -> impl Summary {
	Tweet {
		username: "tcm",
		content: "era of abundance...is it really?",
		retweet: false,
		reply: false,
	}
}
```

This is mostly useful in cases of **Iterators** and **Closures**. For example an iterator for say a Vector which only the compiler knows implements the Iterator trait and returns the trait. The caller just needs to call the methods `next()`, `prev()`, `begin()`, `end()` and so on to interact with the actual type.

**NOTE** - _When using `impl <trait>` syntax for returning you can specify / return only a single type. For example the following won't work -_

```
fn return_summarizable(switch: bool) -> impl Summary {
	if switch {
	    Tweet {
		username: String::from("tcm"),
		content: String::from("era of abundance... is it really?"),
		retweet: false,
		reply: false,
	    }
	} else {
	    NewsArticle {
		author: String::from("the common man"),
		location: String::from("news paper"),
		content: String::from("era of abundance... is it really?"),
		headline: String::from("era of abundance... is it really?"),
	    }
	}
}
```

This doesn't work due to restrictions around how `impl <trait>` has been implemented in the compiler. More on how to get around this is in Chapter 17.

## Using trait bounds to conditionally implement methods

We can implement methods conditionally for types that implement certain specified traits. 

Example - The `new` function is available for all types of `T`. But the `display` is only implemented only for types that implement the `Display` and `PartialOrd` traits.

```
use std::cmp::PartialOrd;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditionally implement methods based on the type of trait
//
impl<T: Display + PartialOrd> Pair<T> {
    fn display(&self) {
        if self.x > self.y {
            println!("pair x {} < pair y => {}", self.x, self.y);
        } else {
            println!("pair y {} < pair x => {}", self.y, self.x);
        }
    }
}

fn main() {
    let p1 = Pair::new(5, 10);
    println!("DISPLAY");
    p1.display();
}
```

