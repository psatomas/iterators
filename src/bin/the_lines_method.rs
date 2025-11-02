use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("story.txt")?;

    for line in contents.lines() {
        println!("{line}");
    }

    Ok(())
}