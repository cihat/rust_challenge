/*
* A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
* You may recognize these from other programming languages. Let’s jump into how they work in Rust.
 */

use std::io;

fn main() {
  println!("Hello, world!");

  //* Tuple
  let tup_variable: (i16, f64, u8) = (500, 6.4, 1);
  let five_hundred = tup_variable.0;
  let six_point_four = tup_variable.1;
  let one = tup_variable.2;
  print!("{five_hundred}-{six_point_four}-{one}");

  //* array */
  let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  // let array_variable: [i32; 5] = [1, 2, 3, 4, 5];
  // println!("{array_variable}");

  // let a = [3; 5];
  // println!("{}", a)

  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}
