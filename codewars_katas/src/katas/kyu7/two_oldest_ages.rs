pub fn two_oldest_ages() {
  let input_array: Vec<u8> = vec![1, 5, 87, 45, 8, 8];  

  solution(&input_array);
}

pub fn solution(ages: &[u8]) -> [u8; 2] {
  let mut sorted_ages = ages.to_vec();
  sorted_ages.sort_by(|a, b| b.cmp(a));

  [sorted_ages[1], sorted_ages[0]]
}
