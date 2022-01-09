use std::fs::File;
use std::io::{self};

pub fn task_one(mut lines: io::Lines<io::BufReader<File>>) -> i32 {
  let xs: Vec<i32> = lines
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .filter_map(|s| s.parse().ok())
    .collect();

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
