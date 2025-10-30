fn main() {
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    let name_lengths= names
        .iter()
        .map(|name | name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect::<Vec<usize>>();

    println!("{name_lengths:?}");    
}
