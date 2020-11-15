use std::fmt;

pub fn say_hello() {
  println!("Hello world from Rust!");
}

pub fn print(limit: u8) {
  let numbers= generate_sequence(limit);
  output_sequence(&numbers);
}


// NOTE: Last checkpoint: change signature argument input from Vec to reference of a slice
fn output_sequence(numbers: &[Number]) {
  for n in numbers {
    println!("{}", n);
  }
}
 
fn generate_sequence(limit: u8)-> Vec<Number> {
  let mut numbers = Vec::new();
  for n in 1..=limit {
    numbers.push(Number { name: String::from("auto"), value: n })
  }

  numbers
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