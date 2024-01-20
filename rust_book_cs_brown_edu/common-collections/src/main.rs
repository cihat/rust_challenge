use std::collections::HashMap;

fn main() {
  // let mut v = vec![100, 32, 57];
  //   for i in &mut v {
  //       *i += 50;
  //   }

  //   println!("{:?}", v);

  // let hello = "Здравствуйте";

  // for c in hello.chars() {
  //   println!("{}", c);
  // }

  // HashMap

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);

  println!("{score}");

  scores.entry(String::from("Yellow")).or_insert(40);
  scores.entry(String::from("Blue")).or_insert(50);

  //  println!("{:?}", scores)

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

println!("output: {:?}", map);
}
