fn main() {
    iterate_vector();
    iterator_demonstration();
    iterator_sum();
}

fn iterate_vector() {
    let v1 = vec![1, 2, 3, 4, 5];
    // NOTE there is no need to declare iterator (iter) as mutable
    let iter = v1.iter();
    for val in iter {
        println!("value: {}", val);
    }
}

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut iter = v1.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4];
    let iter = v1.iter();
    let total: i32 = iter.sum();
    println!("Sum of vector {:#?} is {}", v1, total);
}
