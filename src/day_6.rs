use std::fs::File;
use std::io::{self};

fn get_timers(mut lines: io::Lines<io::BufReader<File>>, epoch: u16) -> u64 {
  let mut timers = lines
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .fold(vec![0; 9], |mut accum, s| {
      if let Ok(timer) = s.parse::<usize>() {
        accum[timer] += 1;
      }

      accum
    });

  for _ in 0..epoch {
    timers.rotate_left(1);
    timers[6] += timers[8];
  }

  timers.iter().sum()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u64 {
  get_timers(lines, 80)
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> u64 {
  get_timers(lines, 256)
}
