#[allow(dead_code)]
pub fn main() {
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  let separator = 3;

  even_numbers(&numbers, separator);
}

// fn even_numbers(array: &Vec<i32>, mut number: usize) -> Vec<i32> {
//   let mut answer = Vec::<i32>::new();

//   for &n in array.iter().rev() {
//     if n % 2 == 0  {
//       answer.push(n);
//       number -= 1;
//     }

//     if number == 0 { break; }
//   }

//   answer.reverse();

//   answer
// }
fn even_numbers(original: &[i32], number: usize) -> Vec<i32> {
  let even_numbers: Vec<i32> = original
    .iter()
    .rev()
    .filter(|&x| x % 2 == 0)
    .cloned()
    .collect();

  let mut result: Vec<i32> = even_numbers.iter().take(number).cloned().collect();

  result.reverse();

  println!("{:?}", result);

  result
}
