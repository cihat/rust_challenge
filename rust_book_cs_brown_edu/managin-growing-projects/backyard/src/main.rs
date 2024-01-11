use crate::garden::front_of_house::front_of_house::hosting::add_to_waitlist;
use crate::garden::vegetables::Asparagus;
use crate::garden::front_of_house::{eat_at_restaurant, front_of_house};
// use crate::garden::*;

pub mod garden;

fn main() {
  let plant = Asparagus {};

  println!("I'm growing {:?}!", plant);

  // crate::front_of_house::hosting::add_to_waitlist();
  add_to_waitlist();

  eat_at_restaurant();
}
