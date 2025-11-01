fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let total: i32 = numbers.iter().sum();
    println!("{total}");

    let product: i32 = numbers.iter().product();
    println!("{product}");

    let max = numbers.iter().max().unwrap();
    println!("{max}");

    let min = numbers.iter().min().unwrap();
    println!("{min}");

    let count = numbers.iter().count();
    println!("{count}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];

    let sum = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
    println!("{sum:?}");

    let max = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .reduce(|accum, current| accum.max(current));
    println!("{max:?}");
}