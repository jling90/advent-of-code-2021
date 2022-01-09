use std::fs::File;
use std::io::{self};

fn input_to_vec(mut lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
  lines
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .filter_map(|s| s.parse().ok())
    .collect()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> i32 {
  let xs: Vec<i32> = input_to_vec(lines);
  let (min, max) = (xs.iter().min().unwrap(), xs.iter().max().unwrap());

  // TODO (perf): Consider binary search
  (*min..*max)
    .map(|d| {
      xs.iter().fold(0, |total_distance, distance| {
        total_distance + (distance - d).abs()
      })
    })
    .min()
    .unwrap()
}

fn gauss_sum(n: i32) -> i32 {
  (n * (n + 1)) / 2
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> i32 {
  let xs: Vec<i32> = input_to_vec(lines);
  let (min, max) = (xs.iter().min().unwrap(), xs.iter().max().unwrap());

  // TODO (perf): Consider binary search
  (*min..*max)
    .map(|d| {
      xs.iter().fold(0, |total_distance, distance| {
        total_distance + gauss_sum((distance - d).abs())
      })
    })
    .min()
    .unwrap()
}
