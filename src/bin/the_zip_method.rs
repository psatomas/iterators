fn main() {
    let first_names = ["Casey", "Robert", "Cargo"];
    let last_names = ["Johnson", "Smith", "Rustman", "Digman"];

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{first_name} {last_name}");
    }

    let complete_names = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| format!("{first_name} {last_name}"))
        .collect::<Vec<String>>();

    println!("{complete_names:?}");
}