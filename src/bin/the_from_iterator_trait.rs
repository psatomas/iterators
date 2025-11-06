use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}
fn main() {
    let fifty_numbers = 1..=50;

    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{results:?}");

    let results: Vec<_> = fifty_numbers.clone().collect();
    println!("{results:?}");

    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers);
    println!("{unique_set:?}");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    // let playlist: Playlist = Playlist::from_iter(songs);
    // println!("{playlist:?}");

    let playlist = songs.into_iter().collect::<Playlist>();
}