fn main() {
    let numbers = vec![100, 14, 45, 345, 94, 130, 5006, 938];
    println!("largest number is {}", largest_i32(&numbers));

    let chars = vec!['a', 'b', 'd', 'c', 'x', 'l', 'z'];
    println!("largest character is {}", largest_char(&chars));

    println!("largest of numbers (templated): {}", largest(&numbers));
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

// Eliminate duplication by parameterization of types
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// For this signature -
//
// fn largest<T>(list: &[T]) -> &T
//
// The following error is shown -
//
//error[E0369]: binary operation `>` cannot be applied to type `&T`
//  --> src/main.rs:37:17
//   |
//37 |         if item > largest {
//   |            ---- ^ ------- &T
//   |            |
//   |            &T
//   |
//help: consider restricting type parameter `T`
//   |
//34 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//   |             ++++++++++++++++++++++
//
//For more information about this error, try `rustc --explain E0369`.
//error: could not compile `generic_functions` due to previous error
//
