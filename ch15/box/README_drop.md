## Drop Trait

The `Drop` trait is implemented on types that need clean up / release of resources when they go out of scope. Example file handles, sockets, locks, instance on heap to call free and so on. These types implement the `Drop` trait to perform cleanup. The Rust compiler calls the `drop` method when the values go out of scope. There is no need for the programmer to introduce drop/deallocation logic.

The `Drop` trait has one method `drop` which takes a `&mut self` as argument and performs cleanup.

Unlike the `Deref` trait in `std::ops::Deref` the `Drop` trait is in the prelude and doesn't need to be brought into scope.

The `drop` methods are called in reverse order of their construction -

```
#[derive(Debug)]
struct CustomStruct {
    v: String,
}

impl Drop for CustomStruct {
    fn drop(&mut self) {
        println!("dropping custom struct: {}", self.v);
    }
}

fn main() {
    {
        let c = CustomStruct {
            v: String::from("...c..."),
        };
        println!("custom struct: {:?}", c);
        let d = CustomStruct {
            v: String::from("...d..."),
        };
        println!("custom struct: {:?}", d);
    }
}
```

**Output:**

```
custom struct: CustomStruct { v: "...c..." }
custom struct: CustomStruct { v: "...d..." }
dropping custom struct: ...d...
dropping custom struct: ...c...
```

## Dropping value early with std::mem::drop

Sometimes it is necessary to cleanup earlier than the scope ends. For such cases it is useful to drop an instance by manually calling the `drop` function. We cannot call the `<instance>.drop()` method directly. We could call the `drop(<instance>)` function instead.

```
#[derive(Debug)]
struct CustomSmartPointer {
    v: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping: {}", self.v);
    }
}

fn main() {
    {
        let _c = CustomSmartPointer {
            v: String::from("c"),
        };

        let _d = CustomSmartPointer {
            v: String::from("d"),
        };

        println!("c and d have been initialized");
        drop(_c);
        println!("scope ending");
    }
    println!("program ending");
}
```

Here you'll see the value of `c` is dropped sooner than `d` which is dropped at the end of the scope.

**Output:**

```
c and d have been initialized
dropping: c
scope ending
dropping: d
program ending
```
