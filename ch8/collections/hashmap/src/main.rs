use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Team-A", 50);
    scores.insert("Team-B", 60);

    println!("scores hash map initialized");
    let score = scores.get("Team-A");
    match score {
        Some(score) => println!("Team-A score: {}", score),
        None => println!("No such team: Team-A"),
    };

    println!("Scores:");
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Ownership
    let field_name = String::from("PC");
    let field_value = String::from("Personal Computer");
    let mut acronyms = HashMap::new();
    acronyms.insert(field_name, field_value);
    // field_name and field_value are not usable from here onwards.

    println!("Acronyms:");
    for (k, v) in &acronyms {
        println!(" => {}: {}", k, v);
    }

    // Overwriting
    println!("=== Updating a HashMap: ===");
    println!("Overwriting:");
    let mut h = HashMap::new();
    h.insert("Blue", 50);
    h.insert("Blue", 100);
    println!("{:?}", h);

    // Adding key-value only when key does not exists
    println!("Update only when key is not present:");
    let mut h = HashMap::new();
    let default = 500;
    h.insert("Blue", 50);
    h.entry("Yellow").or_insert(default);
    h.entry("Blue").or_insert(default);
    println!("{:?}", h);
}
