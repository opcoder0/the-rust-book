use std::ops::Deref;

#[derive(Debug, PartialEq)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);

    assert_eq!(*y, 5);

    // Without the implementation of the `Deref` trait the following error
    // is thrown -
    //
    //     error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
}
