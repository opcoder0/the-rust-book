# Object Oriented Programming Features of Rust

- Rust supports encapsulation and object notation via `struct` and `enum`. (Though these are not called objects.)
- Rust does not have inheritance. 
- Rust implements polymorphism via `Traits`.
- Data hiding is implemented via absence of the `pub` keyword which makes the member / method public.


## Important example illustrating difference between using Generic with trait bound vs. Trait Objects

To implement a `gui` which allows you to draw things on the screen -

- we define a trait
```
pub trait Draw {
    fn draw(&self);
}
```

### Using Trait Objects

Any GUI component that implements `Draw` trait can be drawn on the screen.

- Screen definition

```
struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

Screen has one method `run` which renders the GUI -

```
impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
		    component.draw();
		}
	}
}
```

### Using Generic with Trait Bounds

This behaves differently from the generic type that is trait bound. Example - 

```
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> 
where:
    T: Draw,
{
	pub fn draw(&self) {
		for component in self.components.iter() {
		    component.draw();
		}
	}

}
```

**NOTE** This restricts the screen to hold components of the same type. All vector of `Rectangle` or vector of `Ellipse` etc.

## Implementing the Trait

Implement a Button -

```
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
	fn draw(&self) {
	    // implement drawing of button
	}
}
```

Implement a SelectBox -

```
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options : Vec<String>,
}

impl Draw for Button {
	fn draw(&self) {
	    // implement drawing of select box
	}
}
```

In main -

```
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 75,
                options: vec!["yes", "no", "maybe"],
            }),
            Box::new(Button {
                width: 200,
                height: 60,
                label: String::from("Ok"),
            }),
        ],
    };
}
```

Here the screen can hold any value implementing `Draw`. Even the ones we don't know.

## Performance implication

For Generics the compiler monomorphises the code and generates code for every concrete type there is for the given T. And the compiler knows at compile time which function is being called. These substituitions happen at compile time.

For Trait objects the object is not known until runtime. This uses the pointers to functions (vtable) to call `draw` on a type implementing the trait. This incurs some runtime performance overhead.
