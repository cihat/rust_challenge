//fn calculate_length(s: &String) -> usize {
//  s.len()
//}

//fn change (some_string: &mut String) {
//    some_string.push_str(", world");
//}

fn main() {
  //let s1 = String::from("hello");

  //let len = calculate_length(&s1);

  //println!("The length of '{}' is {}.", s1, len);

  // let mut s = String::from("Hello");
  // change(&mut s);

  /* let mut s = String::from("hello");

  let r1 = &mut s;
  let r2 = &mut s;

  println!("{}, {}", r1, r2);


  // for the sollution we can create a new scope
  let mut s = String::from("hello");

  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let r2 = &mut s;
  */

  /*
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  let r3 = &mut s; // BIG PROBLEM

  println!("{}, {}, and {}", r1, r2, r3);

  ////!


  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{}", r3);
   */

  // At any given time, you can have either one mutable reference or any number of immutable references.
  // References must always be valid.
}
