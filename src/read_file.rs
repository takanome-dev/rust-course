// iterate through the file `lines` and print every other lines
fn iterate_file() {
  let file = std::fs::read_to_string("lines").unwrap();

  file
    .lines()
    .enumerate()
    .filter(|(idx,_)| idx % 2 == 0)
    .for_each(|(_, line)| println!("{}", line));
}

// same as above but skip the first two lines
fn main() {
  let file = std::fs::read_to_string("lines").unwrap();

  file
    .lines()
    .enumerate()
    .filter(|(idx,_)| idx % 2 == 0)
    .skip(2)
    .for_each(|(_,line)| println!("{}", line));
}