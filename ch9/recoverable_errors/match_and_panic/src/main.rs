use std::fs::File;

fn main() {
    let result = File::open("greeting.txt");
    let greeting_file = match result {
        Ok(f) => f,
        Err(err) => panic!("Cannot open greeting.txt {}", err),
    };
    println!("Greeting file has been opened: {:?}", greeting_file);
}
