use std::fs::File;
use std::io::{self};

fn input_to_grid(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
  lines
    .map(|l| {
      l.unwrap()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect()
    })
    .collect()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
  let grid = input_to_grid(lines);
  let grid_width = grid[0].len();
  let mut count = 0;

  for (row_idx, row) in grid.iter().enumerate() {
    for (col_idx, n) in row.iter().enumerate() {
      let left = if col_idx > 0 { row[col_idx - 1] } else { 10 };
      let right = if col_idx < grid_width - 1 {
        row[col_idx + 1]
      } else {
        10
      };

      let up = if row_idx > 0 {
        grid[row_idx - 1][col_idx]
      } else {
        10
      };

      let down = if row_idx < grid_width - 1 {
        grid[row_idx + 1][col_idx]
      } else {
        10
      };

      if left > *n && right > *n && up > *n && down > *n {
        count += *n + 1;
      }
    }
  }

  count
}
