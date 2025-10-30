fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("Square:{number}");
    }
}