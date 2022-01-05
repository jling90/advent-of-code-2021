use std::cmp::min;
use std::fs::File;
use std::io::{self};

#[derive(Debug)]
struct Line {
  start: [i32; 2],
  end: [i32; 2],
}

impl Line {
  fn is_straight(&self) -> bool {
    let [x1, y1] = self.start;
    let [x2, y2] = self.end;

    x1 == x2 || y1 == y2
  }

  fn indices(&self) -> Vec<[i32; 2]> {
    let [x1, y1] = self.start;
    let [x2, y2] = self.end;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    // Coordinates are not sorted, i.e.
    // it is possible for: y1 > y2.
    let x_min = min(x1, x2);
    let y_min = min(y1, y2);

    if self.is_straight() {
      // horizontal
      if dx == 0 {
        return (y_min..y_min + dy + 1).map(|y| [x1, y]).collect();
      }

      // vertical
      return (x_min..x_min + dx + 1).map(|x| [x, y1]).collect();
    }

    if dx != dy {
      panic!("Non-straight lines should be diagonal {}, {}", dx, dy)
    }

    (0..dx + 1).map(|idx| [x_min + idx, y_min + idx]).collect()
  }
}

fn input_to_line(line: String) -> Line {
  let mut coords = line.split(" -> ");
  let (start_str, finish_str) = (coords.next().unwrap(), coords.next().unwrap());
  let (mut start, mut finish) = (start_str.split(","), finish_str.split(","));

  Line {
    start: [
      start.next().unwrap().parse().unwrap(),
      start.next().unwrap().parse().unwrap(),
    ],
    end: [
      finish.next().unwrap().parse().unwrap(),
      finish.next().unwrap().parse().unwrap(),
    ],
  }
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> i32 {
  let mut overlap_count = 0;

  // FIXME: avoid hardcoding grid size.
  let grid_width = 1000;

  let mut grid = vec![0; grid_width * grid_width];

  for line in lines {
    if let Ok(n) = line {
      let l = input_to_line(n);

      if !l.is_straight() {
        continue;
      }

      for [x, y] in l.indices() {
        grid[x as usize + grid_width * y as usize] += 1;

        if grid[x as usize + grid_width * y as usize] == 2 {
          overlap_count += 1;
        }
      }
    }
  }

  overlap_count
}