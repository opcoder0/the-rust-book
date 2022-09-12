## Generics

Parameterized types or templates. Example -

```
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

Read as - 

__The `Result` enum is generic over two types `T` and `E`__

Generics can be -

- generic functions
- generic structs
- generic enums
- generic methods of structs/enums


```
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
	    &self.x
	}
}
```

We can also have constrained methods for generic types. 

```
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}
```

For example, the example above sets the constraint that only `Point` that is of type `f32` has a method `distance_from_origin` available to it. Not the one that is of any other type (for instance `f64`).


## Mixups

Generic type parameters are not always the same as those you define in your struct's method signatures.


