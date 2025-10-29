use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }

        
    }

    counts
}

fn main() {
    println!(
        "{:?}",
        count_characters("Sally sells sea shells by the sea shore"));
}