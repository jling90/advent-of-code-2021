use std::fs::File;
use std::io::{self};

/*
 * https://adventofcode.com/2021/day/1
 *
 * Read through a file, check if last line (as int)
 * is larger than the previous value and return number
 * of times there was an increase.
 */

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> String {
  let mut n_increments = 0;
  let mut last: Option<i32> = None;

  for line in lines {
    if let Ok(n) = line {
      if let Ok(new_val) = n.parse::<i32>() {
        if let Some(value) = last {
          if new_val > value {
            n_increments += 1;
          }
        }

        last = Some(new_val);
      }
    }
  }

  n_increments.to_string()
}

/*
 * https://adventofcode.com/2021/day/1#part2
 *
 * Like the previous task but using a sliding window of three.
 */
pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> String {
  let mut last_window_sum: Option<i32> = None;
  let mut n_increments = 0;

  let parsed_lines: Vec<i32> = lines
    .filter_map(|line| {
      if let Ok(n) = line {
        if let Ok(new_val) = n.parse::<i32>() {
          Some(new_val)
        } else {
          None
        }
      } else {
        None
      }
    })
    .collect();

  let mut i = 0;

  while i < parsed_lines.len() - 2 {
    let current_window_sum: i32 = parsed_lines[i..i + 3].iter().sum();

    if let Some(last_sum) = last_window_sum {
      if current_window_sum > last_sum {
        n_increments += 1;
      }
    }

    last_window_sum = Some(current_window_sum);

    i += 1;
  }

  n_increments.to_string()
}
