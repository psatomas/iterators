fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];

    for number in &my_vector {
        println!("{number}");
    }

    let cities = vec![String::from("Phoenis"), String::from("Dallas")];
      
    for city in &cities {
        println!("{city}");
    }

    println!("{cities:?}")
}