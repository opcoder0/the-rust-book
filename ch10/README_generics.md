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

```
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn y(&self) -> &Y1 {
        &self.y
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 3.5, y: 4.5 };
    let q: Point<i32, i32> = Point { x: 4, y: 5 };

    println!("p: {:?}", p);
    let mixup = p.mixup(q);
    println!("mixup: {:?}", mixup);
}
```

Here the `mixup` method works on two kind of `Point` concrete types.

_NOTE_ - The `self` is borrowed into `mixup`. So `p` is consumed and cannot be used after the call to `mixup`.
