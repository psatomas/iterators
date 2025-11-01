fn main() {
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<_> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();

    println!("{attendees:?}");
}