fn main () {
  let tests = vec![
    vec![9, 8, 7],
    vec![5, 3, 2],
    vec![6, 6, 7]
  ];

  let t0 = &tests[0];
  println!("{:?}", find_max_index(t0));
}
// pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    
// }

fn find_max_index (input: &Vec<u64>) -> u64 {
  let mut max = input[0];
  let mut max_index = 0;
  for (index, &val) in input.iter().enumerate() {
    if val > max {
      max_index = index as u64;
    }
  }
  max_index
} 