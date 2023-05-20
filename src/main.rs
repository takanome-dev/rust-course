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

fn main() {
  let foo = Color::Blue;

  print!("foo is green: {:?}\n", foo.is_green());
  print!("foo is green parts: {:?}\n", foo.is_green_parts());
}