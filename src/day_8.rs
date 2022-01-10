use std::fs::File;
use std::io::{self};

fn input_to_pairs(lines: io::Lines<io::BufReader<File>>) -> Vec<(Vec<String>, Vec<String>)> {
  lines
    .filter_map(|line| line.ok())
    .map(|line| {
      let mut splits = line.split("|");

      let (signal_pattern, outputs) = (splits.next().unwrap(), splits.next().unwrap());

      (
        signal_pattern
          .split_whitespace()
          .map(|s| s.to_string())
          .collect(),
        outputs.split_whitespace().map(|s| s.to_string()).collect(),
      )
    })
    .collect()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> usize {
  input_to_pairs(lines).iter().fold(0, |sum, (_, outputs)| {
    sum
      + outputs.iter().fold(0, |s, o| {
        s + match o.chars().count() {
          2 | 3 | 4 | 7 => 1,
          _ => 0,
        }
      })
  })
}
