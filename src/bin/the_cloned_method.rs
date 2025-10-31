fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas
        .iter()
        .cloned()
        .filter(|tea| tea.contains("Hot"))
        .collect();
    println!("{more_teas:?}");
}