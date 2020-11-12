use std::fmt;

pub fn say_hello() {
  println!("Hello world from Rust!");
}

pub fn print() {
  let numbers= vec![
    Number{ name: String::from("one"), value: 1 },
    Number{ name: String::from("two"), value: 2 },
    Number{ name: String::from("three"), value: 3 },
    Number{ name: String::from("four"), value: 4 },
    Number{ name: String::from("five"), value: 5 },
  ];
  output_sequence(&numbers);
  output_sequence(&numbers);
}


// NOTE: Last checkpoint: change signature argument input from Vec to reference of a slice
fn output_sequence(numbers: &[Number]) {
  for n in numbers {
    println!("{}", n);
  }
}

struct Number {
  name: String,
  value: u8,
}



impl fmt::Display for Number {
  fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(Hi! Im a Number, my name is {}, and my value is  {})", self.name, self.value)
  }
}