enum Color {
  Red,
  Blue,
  Green,
  Yellow
}

impl Color {
  fn is_green(&self) -> bool {
    if let Color::Green = self {
      return true;
    }
    return false;
  }

  fn is_green_parts(&self) -> bool {
    match self {
        Color::Red => return false,
        Color::Blue => return true,
        Color::Green => return false,
        Color::Yellow => return true,
    }
  }
}

fn print_color(color: Color) {
  match color {
    Color::Red => println!("red"),
    Color::Blue => println!("blue"),
    Color::Green => println!("green"),
    Color::Yellow => println!("yellow")
  };
}

fn log() {
  let foo = Color::Blue;

  print!("foo is green: {:?}\n", foo.is_green());
  print!("foo is green parts: {:?}\n", foo.is_green_parts());
}

struct Custom {
  age: usize,
  name: String
}

enum Item {
  Number(usize),
  String(String),
  MyCustom(Custom)
}

fn append(items: &mut Vec<Item>) {
  items.push(Item::String("Hello Fem!".into()));
}

fn main() {
  let mut items: Vec<Item> = vec![];
  append(&mut items);

  // THIS WILL ERROR COMPARE TO TS which won't
  // let mut items: Vec<usize> = vec![];
  // append(&mut items);
}