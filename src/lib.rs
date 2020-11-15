use std::fmt;

pub fn say_hello() {
  println!("Hello world from Rust!");
}

pub fn print(limit: u8) {
  let numbers= generate_sequence(limit);
  output_sequence(&numbers);
}


fn output_sequence(numbers: &[Number]) {
  for n in numbers {
    println!("{}", n);
  }
}
 
pub fn generate_sequence(limit: u8)-> Vec<Number> {
  (1..=limit)
    .map(|value| Number { name: String::from("auto"), value, })
    .collect()
}

pub struct Number {
  name: String,
  value: u8,
}



impl fmt::Display for Number {
  fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(Hi! Im a Number, my name is {}, and my value is  {})", self.name, self.value)
  }
}

// NOTE: last page: 46 - testing and wrapping up
// NOTE: next page: 47 - Actix Web app

#[test]
fn it_should_generate_at_least_auto_number_one() {
  let list = generate_sequence(1);
  let one = &list[0];
  assert_eq!(String::from("auto"), one.name);
  assert_eq!(1,one.value);
}