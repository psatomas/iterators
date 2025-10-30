use std::collections::HashSet;

fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<i32>>();
    println!("{squares:?}");
    println!("{numbers:?}");
}
