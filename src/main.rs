// Coding an iterator
fn main() {
    let items: Vec<isize> = vec![1, 2, 3]
        .iter() // create the iterator to go over the elements in teh array
        .map(|x| x + 1) // do the plus one'ings
        .collect(); // take the iterator and put it somewhere..

    println!("items {:?}", items);
}