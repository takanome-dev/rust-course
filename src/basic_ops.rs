// Coding an iterator
fn iterator() {
    let items: Vec<_> = vec![1, 2, 3]
        .iter()// create the iterator to go over the elements in teh array
        .map(|x| x + 1) // do the plus one'ings
        .collect(); // take the iterator and put it somewhere..

    println!("items {:?}", items);
}

// This is how `collect` works under the hood
fn how_collect_work() {
    // data lives for the duration of main
    let data = vec![1, 2, 3];

    let mut foo = data.iter().map(|x| x + 1);

    // it creates a new vector (it should be mutable)
    let mut new_vector = vec![];

    // go through the iterator(foo) one at the time
    while let Some(x) = foo.next() {
        // push the mapped value into the new vector 
        new_vector.push(x)
    }

    println!("foo {:?}", new_vector)
}


fn test_your_understanding() {
    // What is value?
    let value: usize = vec![1, 2, 3]
        .iter()
        .sum();

    println!("sum {:?}", value); // expected to be 6

    let how_many_times: usize = vec![1, 2, 3]
    .iter()
    .skip(2)
    .count();

    println!("count {:?}", how_many_times); // expected to be 1

    // what will i print?
    vec![1,2,5,9,4]
    .iter()
    .skip(2)
    .take_while(|&&x| x > 4)
    .for_each(|x| println!("{}", x)); // will print 5 and 9

    let what_about_this: usize = vec![1, 2, 3]
        .iter()
        .filter(|x|  *x % 2 == 0)
        .count();

    println!("what_about_this {:?}", what_about_this); // return 1
}

fn main() {}