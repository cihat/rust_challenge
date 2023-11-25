fn another_function() {
  println!("Another function.");
}

fn main() {
    println!("Hello, world!");

  another_function();
  print_labeled_measurement(5, 'A');

  let x = plus_one(32766);
  println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i16) -> i16 {
  return x + 1;
}
