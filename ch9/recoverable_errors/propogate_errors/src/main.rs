use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    println!("read_username: using expanded match conditions");
    match read_username() {
        Ok(u) => println!("username: {}", u),
        Err(e) => panic!("Cannot read username from file: {}", e),
    }

    println!("read_username: using ? operator");
    match read_username_shortcut() {
        Ok(u) => println!("username: {}", u),
        Err(e) => panic!("Cannot read username from file: {}", e),
    }

    println!("read_username: using ? operator - further shortened");
    match read_username_shortcut_further() {
        Ok(u) => println!("username: {}", u),
        Err(e) => panic!("Cannot read username from file: {}", e),
    }

    println!("read_username: further shortened using fs::read_to_string");
    match read_username_shortcut_further_v2() {
        Ok(u) => println!("username: {}", u),
        Err(e) => panic!("Cannot read username from file: {}", e),
    }
}

fn read_username() -> Result<String, io::Error> {
    let result = File::open("username.txt");
    let mut username_file = match result {
        Ok(f) => f,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    let result = username_file.read_to_string(&mut username);
    match result {
        Ok(_) => return Ok(username),
        Err(err) => Err(err),
    }
}

fn read_username_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// The shortcut function can be further simplified/shortened
fn read_username_shortcut_further() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// The file read operation to string is very common so the `read_to_string` is available
// directly in `fs` module. So this function can be further shortened as -
fn read_username_shortcut_further_v2() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
