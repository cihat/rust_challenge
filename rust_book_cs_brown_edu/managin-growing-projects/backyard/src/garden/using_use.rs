mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

use std::collections::HashMap;

use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
  add_to_waitlist();

  // HashMap()

  
}


use std::fmt;
use std::io::{self, Write};

fn function1() -> fmt::Result {
    // --snip--

    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}


mod front_of_house_three {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

pub use front_of_house_three::hosting;
// use std::io::{self, Write};

pub fn eat_at_restaurant_two() {
  hosting::add_to_waitlist();
}
