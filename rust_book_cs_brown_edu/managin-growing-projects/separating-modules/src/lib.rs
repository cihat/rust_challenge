mod front_of_house;

use std::collections::BTreeMap;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  BTreeMap::new();
}
