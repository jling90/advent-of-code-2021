use std::fs::File;
use std::io::{self};

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> i32 {
  let mut x = 0;
  let mut y = 0;

  for line in lines {
    if let Ok(text) = line {
      let mut splits = text.split_whitespace();

      // First split is the direction, second is the magnitude
      if let (Some(direction), Some(magnitude_str)) = (splits.next(), splits.next()) {
        if let Ok(magnitude) = magnitude_str.parse::<i32>() {
          match direction {
            "up" => x -= magnitude,
            "down" => x += magnitude,
            "forward" => y += magnitude,
            _ => (),
          }
        }
      }
    }
  }

  return x * y;
}
