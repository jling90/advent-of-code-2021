use std::fs::File;
use std::io::{self};

fn parse_grid_line(line: Result<&str, &std::io::Error>) -> Vec<u8> {
  line
    .unwrap()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect::<Vec<u8>>()
}

fn parse_bingo_input(mut lines: io::Lines<io::BufReader<File>>) -> (Vec<u8>, Vec<Vec<u8>>) {
  let mut grids: Vec<Vec<u8>> = vec![];

  // Read 'guesses' from the first line of input
  let numbers = lines
    .next()
    .unwrap()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap())
    .collect::<Vec<u8>>();

  // Skip an empty line of input
  lines.next();

  // Calculate size of grids
  let mut grid_iter = lines.take_while(|line| line.is_ok()).peekable();
  let grid_size = parse_grid_line(grid_iter.peek().unwrap().as_deref()).len();

  // Grab grids until lines is exhausted
  loop {
    grids.push(
      grid_iter
        .by_ref()
        .take(grid_size)
        .flat_map(|line| parse_grid_line(line.as_deref()))
        .collect(),
    );

    // Skip an empty line following each grid
    // and bail out at EOF
    if grid_iter.next().is_none() {
      break;
    };
  }

  (numbers, grids)
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
  let (numbers, grids) = parse_bingo_input(lines);
  println!("input: {:?}, grid: {:?}", numbers, grids);
  1337
}
