use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("greeting.txt");
    let handle = match result {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(f) => f,
                Err(err) => panic!("Cannot create file: {}", err),
            },
            other_error => panic!("Cannot open file: {}", other_error),
        },
    };
    println!("file handle: {:?}", handle);
}
