fn main() {
    let performers = ["Rustful Five", "Rust in Peace", "Rustin Bieber"];

    let last: &str = performers.iter().last().unwrap();
    println!("{last}");

    let second = performers.iter().nth(1).unwrap();
    println!("{second}");

    let last = performers.iter().nth_back(0).unwrap();
    println!("{last}");

    let second_to_last = performers.iter().nth_back(1).unwrap();
    println!("{second_to_last}");

    let target_index = performers
        .iter()
        .position(|element| *element == "Rustin Bieber");
    println!("{:?}", target_index);
}