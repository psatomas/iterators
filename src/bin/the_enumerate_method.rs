fn main() {
    let applicants = vec!["Bob", "Rob", "Cob", "Alex", "Piers", "John"];

    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    println!("{winners:?}");
}