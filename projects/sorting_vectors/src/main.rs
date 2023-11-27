#[path = "./sort_of_structs.rs"]
mod sort_of_structs;
fn main() {
  // fn_sort();
  // fn_sort_vector_of_floats();
  sort_of_structs::sort_of_structs();
}

fn fn_sort() {
  let mut vec = vec![1, 5, 10, 2, 15, 324234234];

  vec.sort();

  assert_eq!(vec, vec![1, 2, 5, 10, 15, 324234234]);
}

fn fn_sort_vector_of_floats() {
  let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

  vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
  assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5])
}
