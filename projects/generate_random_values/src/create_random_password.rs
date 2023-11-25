use rand::Rng;

pub fn create_random_password() {
  use rand::Rng;
  const CHARTSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789)(*&^%$#@!~";
  const PASSWORD_LEN: usize = 30;
  let mut rng = rand::thread_rng();

  let password: String = (0..PASSWORD_LEN)
    .map(|_| {
      let idx = rng.gen_range(0..CHARTSET.len());
      CHARTSET[idx] as char
    })  
    .collect();
  println!("{:?}", password)
}
