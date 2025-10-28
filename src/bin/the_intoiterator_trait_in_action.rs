use std::collections::HashMap;

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS",2);
    let my_iterator = my_hashmap.into_iter();
}