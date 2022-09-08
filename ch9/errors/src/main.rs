fn main() {
    println!("Hello, world!");
    // panic();
    vec_panic();
}

fn panic() {
    panic!("panicccc!");
}

fn vec_panic() {
    let v = vec![1, 2, 3];
    println!("vector: {:?}", v);

    v[99];
}
