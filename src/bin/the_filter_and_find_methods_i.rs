
fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();
    println!("{evens:?}");
    println!("{numbers:?}");

    let first_even = numbers
        .into_iter()
        .find(|number| *number % 2 == 0);
    println!("{first_even:?}");

    let first_odd = numbers
        .into_iter()
        .find(|number| *number % 2 != 0);
    println!("{first_odd:?}");

    let nothing = numbers
        .into_iter()
        .find(|number| *number > 100);
    println!("{nothing:?}");

    let last_even = numbers
        .into_iter()
        .rfind(|number| *number % 2 == 0);
    println!("{last_even:?}");

    let last_odd = numbers
        .into_iter()
        .rfind(|number| *number % 2 != 0);
    println!("{last_odd:?}");
}