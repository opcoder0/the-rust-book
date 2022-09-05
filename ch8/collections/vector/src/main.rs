fn main() {
    println!("simple initialization:");
    simple_initialization();
    println!();

    println!("accessing vector element by reference / mutable reference");
    referencing();
    println!();
}

fn simple_initialization() {
    // initialize vector
    println!("initialize vector using Vec::new()");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    for i in v {
        print!("{} ", i);
    }
    println!();

    // initialize vector with vec! macro
    println!("initialize vector using vec! macro");
    let mut v = vec![1, 2, 3, 5, 6];
    println!("vector: (before insert) - {:?}", v);
    v.insert(3, 4);
    println!("vector: (after insert) - index(3) - {:?}", v);
    for i in v {
        print!("{} ", i);
    }
    println!();
}

struct Person {
    name: String,
    email: String,
}

fn referencing() {
    // referencing element -- mutable, value
    let mut people: Vec<Person> = Vec::new();
    let names = ["sb", "pt", "cr", "sq"];
    let emails = [
        "sb@example.org",
        "pt@example.org",
        "cr@example.org",
        "sq@example.org",
    ];
    let mut i: usize = 0;
    while i < names.len() {
        let p: Person = Person {
            name: names[i].to_string(),
            email: emails[i].to_string(),
        };
        people.push(p);
        i += 1;
    }
    for p in &people {
        println!("Name: {}, Email: ({})", p.name, p.email);
    }

    let pk: Person = Person {
        name: "pk".to_string(),
        email: "pk@example.org".to_string(),
    };

    println!("=== Adding one more (pk) ===");
    people.push(pk);
    for p in &people {
        println!("Name: {}, Email: ({})", p.name, p.email);
    }

    let mut p4 = &mut people[4];
    println!("=== changing email of {} ===", p4.name);
    p4.email = "pk@devious.net".to_string();

    println!("=== after email modification ===");
    for p in &people {
        println!("Name: {}, Email: ({})", p.name, p.email);
    }
}
