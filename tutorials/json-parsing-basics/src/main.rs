use serde_json::{Result, Value, json};
use serde::{Serialize, Deserialize};

// fn parsin_json_manually() -> Result<()> {
//   let data = r#"
//     {
//       "name": "JOhn Doe",
//       "age": 43,
//       "phones": [
//         "+44 12312323",
//         "+90 234234343"
//       ]
//     }
//   "#;

//   let v: Value = serde_json::from_str(data)?;

//   println!("PLase call {} at the number {}", v["name"], v["phones"][0]);

//   Ok(())
// }

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
  message: String
}
fn conver_json_to_struct() {
  let raw_json_string = json!({"message": "Hello world!"});
  let my_struct: MyStruct = serde_json::from_str(raw_json_string).unwrap();
}

fn main() {
  
}
