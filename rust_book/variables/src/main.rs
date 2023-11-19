fn main() {
  /*
  let mut x = 5;
  println!("the value of x is: {x}");

  x = 6;
  println!("The value of x is: {x}");

  // Constants
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("the constant value {THREE_HOURS_IN_SECONDS}");

  //Shadowing

  let y = 5;

  let y = y + 1;

  {
    let y = y * 2;
    println!("The value of x in the inner scope is: {y}");
  }

  println!("The value of x is: {y}");
  */

  let spaces = "   ";
  let spaces: usize = spaces.len();
  println!("{spaces}");

  let mut variable_one = "   ";
  println!("{variable_one}");
  variable_one = "testsafd;kl";
  /*
   * we can't let mut variable set different data type
   * variable_one = 100;
   * println!("{variable_one}"); */
}
