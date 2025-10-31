#[derive(Debug, PartialEq, Eq)]
enum CheannelType {
    Comedy,
    News,
    ProgrammingTutoriais
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: CheannelType,
}

fn main() {
    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: CheannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: CheannelType::ProgrammingTutoriais,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: CheannelType::News,
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: CheannelType::ProgrammingTutoriais,
        },
    ];

    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == CheannelType::ProgrammingTutoriais);
    println!("{all_are_rust}");

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == CheannelType::ProgrammingTutoriais);
    println!("{any_are_rust}");
}