fn main() {
    println!("Hello, world!");
    panic_1();
    // vec_panic();
}

fn panic_1() {
    panic_2();
}

fn panic_2() {
    panic_3();
}

fn panic_3() {
    panic_4();
}

fn panic_4() {
    panic();
}

fn panic() {
    panic!("panicccc!");
}

fn vec_panic() {
    let v = vec![1, 2, 3];
    println!("vector: {:?}", v);

    v[99];
}
