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

    if self.is_straight() {
      let dx = (x2 - x1).abs();
      let dy = (y2 - y1).abs();

      // Coordinates are not sorted, i.e.
      // it is possible for: y1 > y2.
      let x_min = min(x1, x2);
      let y_min = min(y1, y2);

      // horizontal
      if dx == 0 {
        return (y_min..y_min + dy + 1).map(|y| [x1, y]).collect();
      }

      // vertical
      return (x_min..x_min + dx + 1).map(|x| [x, y1]).collect();
    }

    return (0..(x1 - x2).abs() + 1)
      .map(|idx| {
        [
          if x1 > x2 { x1 - idx } else { x1 + idx },
          if y1 > y2 { y1 - idx } else { y1 + idx },
        ]
      })
      .collect();
  }
}

fn string_to_line(line: String) -> Line {
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

fn input_to_lines(lines: io::Lines<io::BufReader<File>>) -> impl Iterator<Item = Line> {
  lines.map(|l| string_to_line(l.unwrap()))
}

fn get_overlaps<T>(lines: T) -> i32
where
  T: Iterator<Item = Line>,
{
  // FIXME: avoid hardcoding grid size.
  const GRID_WIDTH: usize = 1000;
  let mut grid = [0; GRID_WIDTH * GRID_WIDTH];

  let mut overlap_count = 0;

  for line in lines {
    for [x, y] in line.indices() {
      grid[x as usize + GRID_WIDTH * y as usize] += 1;

      if grid[x as usize + GRID_WIDTH * y as usize] == 2 {
        overlap_count += 1;
      }
    }
  }

  overlap_count
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> i32 {
  get_overlaps(input_to_lines(lines).filter(|l| l.is_straight()))
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> i32 {
  get_overlaps(input_to_lines(lines))
}
