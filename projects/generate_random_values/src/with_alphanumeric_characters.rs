use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn alphanumeric() {
  let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();

  println!("{}", rand_string);
}
