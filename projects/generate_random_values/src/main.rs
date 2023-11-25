use rand::Rng;
use rand::distributions::{Distribution, Uniform};
#[path = "./with_custom_type.rs"] mod with_custom_type;
#[path = "./create_random_password.rs"] mod custom_type;

fn main() {
  // generate_random_numbers();

  // generate_random_number_within_range();
  with_custom_type::with_custom_type();
  custom_type::create_random_password();
}

fn generate_random_numbers() {
  let mut rng = rand::thread_rng();

  let n1: u8 = rng.gen();
  let n2: u16 = rng.gen();

  println!("Random u8: {}", n1);
  println!("Random u16: {}", n2);
  println!("Random u32: {}", rng.gen::<u32>());
  println!("Random i32: {}", rng.gen::<i32>());
  println!("Random float: {}", rng.gen::<f64>());
}

fn generate_random_number_within_range() {
  let mut rng = rand::thread_rng();
  //println!("Integer: {}", rng.gen_range(0..10));
  //println!("Fload: {}", rng.gen_range(0.0..10.0));

  let die = Uniform::from(1..7);

  loop {
    let throw = die.sample(&mut rng);
    println!("Roll the die: {}", throw);
    if throw == 6 {
      break;
    }
  }
}

fn with_given_distribution() {
  // let mut rng = thread_rng();
  // let normal = Normal::new(2.0, 3.0)?;
  // let v = normal.sample(&mut rng);
  // println!("{} is from a N(2, 9) distribution", v);
  // Ok(());
}
