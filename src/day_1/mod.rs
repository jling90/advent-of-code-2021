use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

// https://adventofcode.com/2021/day/1
pub fn task_one() {
  let mut n_increments = 0;
  let mut last: Option<i32> = None;

  match read_lines("./resources/day-1.txt") {
    Ok(lines) => {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        if let Ok(n) = line {
          match n.parse::<i32>() {
            Ok(new_val) => {
              if let Some(value) = last {
                if new_val > value {
                  n_increments += 1;
                }
              }

              last = Some(new_val);
            }
            Err(_) => (),
          }
        }
      }
    }
    Err(e) => println!("Failed to read file with err:{}", e),
  }

  println!("Number of increments: {}", n_increments);
}
