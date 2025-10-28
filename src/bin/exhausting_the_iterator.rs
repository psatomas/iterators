fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

}