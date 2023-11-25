fn main() {
  // let s: String = String::from("Hello world");
  // let word = first_world(&s);
  //
  // s.clear();
  //
  // let my_string = String::from("hello world");
  //
  //   // `first_word` works on slices of `String`s, whether partial or whole
  //   let word = first_word(&my_string[0..6]);
  //   let word = first_word(&my_string[..]);
  //   // `first_word` also works on references to `String`s, which are equivalent
  //   // to whole slices of `String`s
  //   let word = first_word(&my_string);
  //
  //   let my_string_literal = "hello world";
  //
  //   // `first_word` works on slices of string literals, whether partial or whole
  //   let word = first_word(&my_string_literal[0..6]);
  //   let word = first_word(&my_string_literal[..]);
  //
  //   // Because string literals *are* string slices already,
  //   // this works too, without the slice syntax!
  //   let word = first_word(my_string_literal);
  //
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];

  let result = assert_eq!(slice, &[2, 3]);
  println!("result: {}", result);

}

fn first_world(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
//

// Summary
// The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.
//
// Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
