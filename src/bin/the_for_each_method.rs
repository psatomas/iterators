use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    words.for_each(|word|{
        word.chars().for_each(|character| {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        });
    });

    counts
}

fn main() {
    println!(
        "{:?}",
        count_characters("Sally sells sea shells by the sea shore"));
}