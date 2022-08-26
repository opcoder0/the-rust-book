const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const PI: f32 = 22.0 / 7.0;

fn main() {
    // variables by default are immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Three Hours in Seconds: {THREE_HOURS_IN_SECONDS}");

    // variables can be shadowed.
    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1; // new variable 'x' that shadows the previous declaration
    {
        println!("The value of x in the inner scope (before shadowing): {x}");
        let mut x = x + 2; // new variable x that is mutable
        println!("The value of x in the inner scope: {x}");
        x = x + 9;
        println!("The value of x in the inner scope after incr: {x}");
    }
    println!("The value of x is: {x}");

    {
        let x: f32 = PI;
        println!("Shadow and changing type: {x}");
        let spaces = "     ";
        let spaces = spaces.len();
        println!("Found {spaces} spaces");
    }
}
