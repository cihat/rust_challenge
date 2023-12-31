struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn _build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}

fn main() {
  let mut user1 = User {
    email: String::from("someone@eacmples.com"),
    active: true,
    sign_in_count: 1,
    username: String::from("someusername123"),
  };

  user1.email = String::from("anotheremail@example.com");
  println!("Hello, world!");

  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
