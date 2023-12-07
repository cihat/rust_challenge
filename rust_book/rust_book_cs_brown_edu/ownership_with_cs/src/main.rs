// pub fn main() {
//   let m1 = String::from("Hello");
//   let m2 = String::from("world");
//   let (m1_again, m2_again) = greet(m1, m2);
//   let _s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//   println!("{} {}!", g1, g2);
//   (g1, g2)
// }

fn main() {
  // let m1 = String::from("Hello");
  // let m2 = String::from("world");
  // greet(&m1, &m2); // note the ampersands
  // greet(&m1, &m2);
  // let _s = format!("{} {}", m1, m2);

  // Dereferencing a Pointer Accesses Its Data
  // let mut x = Box::new(1);
  // let _a = *x;
  // *x += 1;

  // let r1 = &x;
  // let _b = **r1;

  // let r2 = &*x;
  // let _c = *r2;

  // let x = 4;
  // let y = x;
  // println!("{}", y);
  // println!("{}", x);

  // let mut s = String::from("Hello");
  // let r1 = &s;
  // let r2 = &s;
  // println!("{}, {}", r1, r2);
  // let r3 = &mut s;
  // println!("{}", r3);

  // let mut v = vec![1, 2, 3];
  // v.push(4);
  // let mut v = vec![1, 2, 3];
  // let num = &v[2];

  // println!("Third elements is {}", *num);
  // v.push(4);

  // let x = 1;
  // let y = 23;
  // let mut x_ref = &x;
  // x_ref = &y;
  // println!("{}", x_ref);

  /************
   * test
   * test
   */
  // let mut v: Vec<i32> = vec![1, 2, 3];
  // let num: &i32 = &v[2];
  // println!("Third elemenjt is {}", *num);
  // println!("Again, the third element is {}", *num);
  // v.push(4);

  let mut v: Vec<i32> = vec![1, 2, 3];
  let num: &mut i32 = &mut v[2];
  *num += 1;
  v.push(4);
  println!("Third element is {}", *num);
}

fn _greet(g1: &String, g2: &String) {
  // note the ampersands
  println!("{} {}!", g1, g2);
}

fn _dereference() {
  let x: Box<i32> = Box::new(-1);
  let x_abs1 = i32::abs(*x); // explicit dereference
  let x_abs2 = x.abs(); // implicit dereference
  assert_eq!(x_abs1, x_abs2);

  let r: &Box<i32> = &x;
  let r_abs1 = i32::abs(**r); // explicit dereference (twice)
  let r_abs2 = r.abs(); // implicit dereference (twice)
  assert_eq!(r_abs1, r_abs2);

  let s = String::from("Hello");
  let s_len1 = str::len(&s); // explicit reference
  let s_len2 = s.len(); // implicit reference
  assert_eq!(s_len1, s_len2);
}
