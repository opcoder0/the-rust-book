#[derive(Debug)]
struct Element {
    name: String,
    value: i32,
}

fn main() {
    println!("simple initialization:");
    simple_initialization();
    println!();

    println!("accessing element from Vec<i32>");
    get_zeroth_element();
    println!();

    println!("accessing element from Vec<Element>");
    get_zeroth_struct_element();
    println!();

    println!("iterating over a vector and modifying elements");
    iterate_vector();
    println!();

    println!("vector dropped");
    vector_dropped();
    println!();
}

fn get_zeroth_element() {
    let v = vec![1, 2, 3, 4, 5];
    println!("vec: {:?}", v);
    println!("getting to the zero-th element");
    let zero = v[0];
    println!("zero-th element {}", zero);
}

fn get_zeroth_struct_element() {
    let mut v: Vec<Element> = Vec::new();
    v.push(Element {
        name: "zero-th".to_string(),
        value: 0,
    });
    v.push(Element {
        name: "first".to_string(),
        value: 1,
    });
    v.push(Element {
        name: "second".to_string(),
        value: 2,
    });
    println!("vector<Element>: {:?}", v);
    println!("getting first element");
    let first: &Element = &v[1];
    println!("first element from vector<Element>: {:?}", first);
    println!("first.name: {}, first.value: {}", first.name, first.value);

    // Accessing the first element directly like this
    // let first = v[1];
    // throws a compilation error saying a move is being attempted
    // the type Element does not have a copy trait implemented.
}

fn simple_initialization() {
    println!("Vec::new() - initialization");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v<i32>: {:?}", v);

    println!("vec! - macro initialization");
    let v = vec![1, 2, 3, 4, 5];
    println!("vec!: {:?}", v);
}

fn iterate_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Doubling vector: {:?}", v);
    for i in &mut v {
        *i *= 2;
    }
    println!("Output vector: {:?}", v);
}

fn vector_dropped() {
    {
        let v = vec![1, 2, 3, 4, 5];
        println!("vec: {:?}", v);
    }
    // vector v is not available here. The vector and all
    // its elements are dropped
}
