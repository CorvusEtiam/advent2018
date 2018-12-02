use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::error::Error;

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
  let file = File::open("input.txt").expect("File not found");
  let buf = BufReader::new(file);
  let data : Vec<i32> = buf.lines()
    .map(| line | line.unwrap())
    .map(| s | s.as_str().parse::<i32>().unwrap())
    .collect();
  
  println!("First part: {}", data.iter().sum::<i32>());
  println!("Second part: {}", part_2(&data));  
}
