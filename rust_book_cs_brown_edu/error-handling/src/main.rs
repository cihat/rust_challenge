use std::{error, fs::File, io::ErrorKind};
use std::io::{self, Read};

fn main() {
  // let v = vec![1, 2, 3];

  // v[99];

  // let greeting_file_result = File::open("hello.txt");

  // let greeting_file = match greeting_file_result {
  //   Ok(file) => file,
  //   Err(error) => panic!("Problem opening the file {:?}", error),
  // };

  // println!("{:?}", greeting_file);

  // let greeting_file_result = File::open("hello2.txt");

  // let greeting_file = match greeting_file_result {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello2.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Problem creating the file: {:?}", e),
  //     },
  //     other_error => {
  //       panic!("Problem opening the file: {:?}", other_error);
  //     }
  //   },
  // };

  // println!("{:?}", greeting_file);

  // let greeting_file = File::open("hello3.txt").unwrap_or_else(|error| {
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello3.txt")
  //       .unwrap_or_else((|error| panic!("Problem creating the file: {:?}", error)))
  //   } else {
  //     panic!("Problem opening the file: {:?}", error);
  //   }
  // });

  // println!("{:?}", greeting_file);

  // let greeting_file =
  //   File::open("hello.txt").expect("hello.txt should be included in this project");

  //   println!("{:?}", greeting_file);

  fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
  }

  println!("{:?}", read_username_from_file());
}
