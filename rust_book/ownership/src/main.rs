// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.

fn main() {
  let mut s = String::from("hell");

  s.push_str(", world!");

  println!("{}", s);

  println!("Hello, world! test 2");

  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}!",  s1, s2);
  
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);

  ownership_with_functions();
}

fn ownership_with_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
