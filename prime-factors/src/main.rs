fn main () {
  sieve_of_eratoshees(2);
}

fn sieve_of_eratoshees (n: usize) {
  let mut result: Vec<usize> = Vec::new();
  let mut list: Vec<usize> = Vec::new();
  list.push(0);
  list.push(0);
  for _ in 2..=n {
    list.push(1);
  }
  println!("{:?}", list);
  for i in 2..=n {
    if list[i] == 1 {
      result.push(i);
      let mut j = i * 2;
      while j < n {
        list[j] = 0;
        j = j + i;
      }
    }
  }
  println!("{:?}", result);
}