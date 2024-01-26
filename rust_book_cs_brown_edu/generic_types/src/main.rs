// fn largest_i32(list: &[i32]) -> &i32 {
//   let mut largest = &list[0];

//   for item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// fn largest_char(list: &[char]) -> &char {
//   let mut largest = &list[0];

//   for item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// fn main() {
//   let number_list = vec![23, 50, 234, 234, 45, 22, 234];

//   let result = largest_i32(&number_list);
//   println!("The largest number is {}", result);

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest_char(&char_list);
//   println!("The largest char is {}", result);
// }

// fn largest<T>(list: &[T]) -> &T {
//   let mut largest = &list[0];

//   for item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// #[derive(Debug)]
// struct Point<T> {
//   x: T,
//   y: T,
// }

// fn main() {
//   let integer = Point { x: 5, y: 10 };
//   let float = Point { x: 1.0, y: 4.0 };

//   println!("{:?}, {:?}", integer, float);
// }

// struct Point<T, U> {
//   x: T,
//   y: U,
// }

// fn main() {
//   let both_integer = Point { x: 5, y: 10 };
//   let both_float = Point { x: 1.0, y: 4.0 };
//   let integer_and_float = Point { x: 5, y: 4.0 };
// }

// struct Point<T> {
//   x: T,
//   y: T,
// }

// impl<T> Point<T> {
//   fn x(&self) -> &T {
//     &self.x
//   }
// }

// impl Point<f32> {
//   fn distance_from_origin(&self) -> f32 {
//     (self.x.powi(2) + self.y.powi(2)).sqrt()
//   }
// }

// fn main() {
//   let p = Point { x: 5, y: 10 };

//   println!("p.y = {}", p.y);
//   println!("p.x = {}", p.x());
//   println!("p.distance_from_origin", p.dis)
// }

// struct Point<X1, Y1> {
//   x: X1,
//   y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//   fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//     Point {
//       x: self.x,
//       y: other.y,
//     }
//   }
// }

// fn main() {
//   let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }
// #[derive(Debug)]
// enum Option_i32 {
//   Some(i32),
//   None,
// }
// #[derive(Debug)]
// enum Option_f64 {
//   Some(f64),
//   None,
// }

// fn main() {
//   let integer = Option_i32::Some(5);
//   let float = Option_f64::Some(5.0);

//   println!("{:?}, {:?}", integer, float);
// }

mod aggregator;
use aggregator::{Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
}
