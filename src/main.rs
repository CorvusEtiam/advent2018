use std::collections::HashSet;

fn part_2(data: &Vec<i32>) -> i32 {
  let mut already_seen : HashSet<i32> = HashSet::new();
  let mut acc : i32 = 0;
  for val in data.iter().cycle() {
    already_seen.insert(acc);
    acc += val.clone();
    if already_seen.contains(&acc) {
      return acc;
    } 
  }
  return acc; 
}

fn main() {
  let buf = include_str!("../inputs/input1.txt");
  let data : Vec<i32> = buf.lines()
    .map(| s | s.parse::<i32>().unwrap())
    .collect();
  
  println!("First part: {}", data.iter().sum::<i32>());
  println!("Second part: {}", part_2(&data));  
}
