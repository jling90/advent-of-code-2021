use std::fs::File;
use std::io::{self};

fn get_timers(mut lines: io::Lines<io::BufReader<File>>, epoch: u16) -> u64 {
  let mut timers = lines
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .fold(vec![0; 10], |mut accum, s| {
      if let Ok(timer) = s.parse::<usize>() {
        accum[timer] += 1;
      }

      accum
    });

  for _ in 0..epoch {
    let mut updates: Vec<i64> = vec![0; 10];

    // Iterate backwards so new items only
    // decrement on next loop.
    for i in (0..timers.len()).rev() {
      if i == 0 {
        updates[8] += timers[i];
        updates[6] += timers[i];
        updates[0] -= timers[i];
      } else {
        if timers[i] > 0 {
          updates[i] -= timers[i];
          updates[i - 1] += timers[i];
        }
      }
    }

    for i in 0..timers.len() {
      timers[i] += updates[i]
    }
  }

  timers.iter().map(|n| *n as u64).sum()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u64 {
  get_timers(lines, 80)
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> u64 {
  get_timers(lines, 256)
}
