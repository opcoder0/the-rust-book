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
