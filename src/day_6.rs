use std::fs::File;
use std::io::{self};

pub fn task_one(mut lines: io::Lines<io::BufReader<File>>) -> usize {
  let mut timers: Vec<u32> = lines
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .filter_map(|s| s.parse().ok())
    .collect();

  for _ in 0..80 {
    for i in 0..timers.len() {
      if timers[i] > 0 {
        timers[i] -= 1;
      } else {
        timers[i] = 6;
        timers.push(8);
      }
    }
  }

  timers.len()
}
