fn main() {
    let fifty_numbers = 1..=50;

    for value in fifty_numbers.clone().take(15).skip(5).step_by(2) {
        print!("{value}/")
    }

    println!("{fifty_numbers:?}");
}