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
