use std::io;

fn main() {
    scalar_datatypes();
    compound_datatypes();
}

fn scalar_datatypes() {
    // scalar and compound types
    // scalar literals for numerals
    let i: u32 = 16_384;
    let j: u32 = 0xffff;
    let k: u32 = 0o455;
    let m: u32 = 0b1110_0000;
    let n: u8 = b'A';
    println!("scalar data types initialization");
    println!("i: {i}, j: {j}, k: {k}, m: {m}, n: {n}");
    let x = 3.4; // f64 (default)
    let y: f32 = 4.5;
    println!("x: {x}, y: {y}");

    // bool takes up one byte and can be true or false
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    // char language's primitive alphabetic type.
    // is 4 bytes in size and represents Unicode
    // scalar value (U+0000 to U+D7FF, U+E000 to U+10FFFF)
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("character prints {c}, {z}, {heart_eyed_cat}");
}

fn compound_datatypes() {
    // rust has two compound primitives
    // - tuples
    // - arrays
    tuples();
    arrays();
}

fn tuples() {
    // tuples have fixed length and cannot grow or shrink
    // they can have multiple types in a group
    let tup: (i32, f32, u32) = (20_000, 3.25, 32_786);
    println!("tuple: {tup:?}");
    let (x, y, z) = tup;
    println!("destructuring elements from the tuple {x}, {y}, {z}");
    let t0 = tup.0;
    println!("element 0 of the tuple: {t0}");
    let t1 = tup.1;
    println!("element 1 of the tuple {t1}");

    // tuples without any names is called "unit".
    let zerotup = ();
    println!("zerotup is called a unit {zerotup:?}");
}

fn arrays() {
    // arrays also are fixed length (known at compile time)
    // all elements are of the same type.
    // arrays are allocated on the stack rather than the heap
    // if you want a type which is more flexible and can grow/shrink in size
    // use Vector.
    let arr = [1, 2, 3, 4, 5];
    println!("u32 arr is an array {arr:?}");

    // an array's type can be declared using the convention
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("i32 arr is an array {arr:?}");

    // initialize the array to contain the same value using the convention
    let arr: [f32; 5] = [3.1415; 5];
    println!("f32 arr with initialized values {arr:?}");

    accessing_array();
}

fn accessing_array() {
    let arr = [
        "Apples",
        "Bananas",
        "Grapes",
        "Oranges",
        "Pineapple",
        "Jackfruit",
    ];
    let mut index: String = String::new();

    println!("arr: {arr:?}");
    println!("Enter the index you would like to access:");
    io::stdin().read_line(&mut index).expect("stdin read error");
    let index: usize = index
        .trim()
        .parse()
        .expect("index must be a number between 0 and 5");
    let element = arr[index];
    println!("Element at {index} is {element}");
}
