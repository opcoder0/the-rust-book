fn main() {
    looping();
}

fn looping() {
    loop_return();
    loop_label();
    loop_while();
    loop_for_1();
    loop_for_2();
    loop_for_3();
}

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("looping result {result}");
}

fn loop_label() {
    let mut i: u8 = 0;
    println!("Loop with label");
    'outer: loop {
        loop {
            if i == 20 {
                break 'outer;
            } else if i == 5 {
                break;
            }
            print!("{i} ");
            i = i + 1;
        }
        println!("-");
        i = i + 1;
    }
    println!();
}

fn loop_while() {
    println!("while loop");
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let mut i: usize = 0;
    while i < 5 {
        println!("Loop index at {i} = {}", arr[i]);
        i = i + 1;
    }
    println!();
}

fn loop_for_1() {
    println!("for loop");
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for element in arr {
        println!("element: {}", element)
    }
    println!();
}

fn loop_for_2() {
    println!("for loop range");
    for i in 1..10 {
        println!("{i}");
    }
    println!();
}

fn loop_for_3() {
    println!("for loop range");
    for i in 1..=10 {
        println!("{i}");
    }
    println!();
}
