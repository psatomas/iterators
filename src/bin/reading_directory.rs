use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{entry_name:?}\n-------------");
            let contents = fs::read_to_string(&entry_name)?;
            println!("{contents}");
        }
    }

    Ok(())
}