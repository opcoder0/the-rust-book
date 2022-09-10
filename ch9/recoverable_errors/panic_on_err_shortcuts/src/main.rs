fn main() {
    let opt: u32 = 3;
    match opt {
        1 => unwrap_ok(),
        2 => unwrap_err(),
        3 => expect_err(),
        _ => panic!("invalid test"),
    }
}

fn unwrap_err() {
    let x: Result<u32, &str> = Err("not a valid number");
    x.unwrap();
}

fn unwrap_ok() {
    // x.unwrap() consumes self and panics on error or returns the contained value
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);
    println!("unwrap ok")
}

fn expect_err() {
    let x: Result<u32, &str> = Err("not a valid number");
    x.expect("test failed");
}
