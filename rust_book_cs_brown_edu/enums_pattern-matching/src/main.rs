#[derive(Debug)]
enum IpAddrKind {
  V4 = 23,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

// let home IpAddr {
//   kind: IpAddrKind::V4,
//   address: String::from("127.0.0.1"),
// }

// let loopback = IpAddr {
//   kind: I
// }

// fn main() {
//   let four = IpAddrKind::V4;

//   route(&four);
//   route(&IpAddrKind::V4);

//   println!("test: {:?}", four);
// }

fn route(ip_kind: &IpAddrKind) {
  println!("{:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("{:?}", self)
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn main() {
  // let m = Message::Write(String::from("Hello"));
  // m.call();

  //   let x: i8 = 5;
  //   let y: Option<i8> = Some(5);

  //   // let sum = x + y;

  //   let five = Some(5);
  //   let six = plus_one(five);
  //   let none = plus_one(None);

  //   println!("six: {:?}, \n none: {:?}", six.unwrap(), none)

  let config_max = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
  }

  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max)
  }

  let coin = Some(4);

  let mut count = 0;
  match coin {
    Some(_) => Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state)
    },
    _ => count += 1,
  }

  let mut count = 0;
  let state = 23;

  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }
}

//None

// When we have a Some value, we know that a value is present and the value is held within the Some.
// When we have a None value, in some sense it means the same thing as null: we don’t have a valid value.
// So why is having Option<T> any better than having null?

// In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value.
// For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:
