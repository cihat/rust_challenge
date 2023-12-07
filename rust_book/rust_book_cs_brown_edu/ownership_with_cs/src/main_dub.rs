fn main() {
  let first = String::from("Ferris");
  let first_clone = first.clone();
  let full = add_suffix(first_clone);
  // println!("{}", first_clone); // borrow of moved value: `first value borrowed here after move
  println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
  name.push_str(" Jr.");
  name
}

// Summary
// Ownership is primarily a discipline of heap management:
//
// All heap data must be owned by exactly one variable.
// Rust deallocates heap data once its owner goes out of scope.
// Ownership can be transferred by moves, which happen on assignments and function calls.
// Heap data can only be accessed through its current owner, not a previous owner.
// We have emphasized not just how Rust's safeguards work, but why they avoid undefined behavior. When you get an error message from the Rust compiler, it's easy to get frustrated if you don't understand why Rust is complaining. These conceptual foundations should help you with interpreting Rust's error messages. They should also help you design more Rustic APIs.
//
//
// 1 These data structures don't use the literal Box type. For example, String is implemented with Vec, and Vec is implemented with RawVec rather than Box. But types like RawVec are still box-like: they own memory in the heap.
// 2 In another sense, ownership is a discipline of pointer management. But we haven't described yet about how to create pointers to anywhere other than the heap. We'll get there in the next section.
