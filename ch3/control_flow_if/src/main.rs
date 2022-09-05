fn main() {
    branching();
}

fn branching() {
    // 'if' and 'else' basic syntax
    let n = 6;
    if n % 4 == 0 {
        println!("{n} is divisible by 4");
    } else if n % 3 == 0 {
        println!("{n} is divisible by 3");
    } else if n % 2 == 0 {
        println!("{n} is divisible by 2");
    }

    // NOTE 'if' and 'else' are expressions in rust
    // so this is valid too
    let x = 5;
    let y = if x % 2 == 0 { "even" } else { "odd" };
    println!("x is {x} and it is a {y} number");
}
