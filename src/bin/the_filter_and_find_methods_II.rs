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

    let good_channels: Vec<String>= channels
        .iter()
        .filter(|channel| channel.channel_type == CheannelType::ProgrammingTutoriais)
        .map(|channel| channel.name.clone())
        .collect();
    println!("{good_channels:?}");

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == CheannelType::ProgrammingTutoriais);
    
    match good_channel {
        Some(channel) => println!("Great choice to watch {channel:?}"),
        None => println!("There was no Rust programming on Tv (literally and metaphorically)"),
    }
}