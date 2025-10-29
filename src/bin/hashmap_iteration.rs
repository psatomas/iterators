use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (_, completion_status) in &mut todos {
        *completion_status = true;
    }

    println!("{todos:?}");
}