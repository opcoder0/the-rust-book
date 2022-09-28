#[derive(Debug)]
struct Tshirt {
    size: char,
    color: String,
}

fn main() {
    map_adaptor();
    filter_adaptor();
}

fn map_adaptor() {
    let v = vec![1, 2, 3, 4];
    let iter = v.iter().map(|x| x + 1);
    // calls the closure above for each element. The vector remains unchanged
    for elem in iter {
        println!("{}", elem);
    }
    println!("vector: {:?}", v);
}

fn filter_adaptor() {
    let tshirts = vec![
        Tshirt {
            size: 'L',
            color: String::from("Blue"),
        },
        Tshirt {
            size: 'M',
            color: String::from("Red"),
        },
        Tshirt {
            size: 'M',
            color: String::from("Pink"),
        },
    ];
    println!("Number of T-Shirts {}", tshirts.len());
    let medium_tshirts: Vec<Tshirt> = tshirts.into_iter().filter(|x| x.size == 'M').collect();
    println!("Number of medium size T-Shirts: {}", medium_tshirts.len());
}
