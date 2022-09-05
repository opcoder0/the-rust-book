fn main() {
    functions_statements_and_expressions();
}

fn functions_statements_and_expressions() {
    // statement performs action and always ends with ; (semicolon)
    // expression evaluates to a value and is not followed by a semicolon
    let x = 5;
    let y = {
        let z = 10;
        z + x
    };
    println!("x:{x}, y:{y}");
    let mut a = 0;
    println!("a initial value: {a}");
    let b = a = 3; // is not what you expect. b is ()
    println!("a:{a}, b:{b:#?}");
    let s = odd_or_even(5);
    println!("5 is an {s} number");
}

// implicit return using the expression.
// earlier returns must have 'return' keyword
// last/final return can be an expression
fn odd_or_even(i: i32) -> String {
    if i % 2 == 0 {
        return "even".to_string();
    }
    "odd".to_string()
}
