use std::collections::HashSet;
use std::fs::File;
use std::io::{self};

fn parse_grid_line(line: Result<&str, &std::io::Error>) -> Vec<u8> {
  line
    .unwrap()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect::<Vec<u8>>()
}

fn parse_bingo_input(mut lines: io::Lines<io::BufReader<File>>) -> (Vec<u8>, Vec<Vec<u8>>, usize) {
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

  (numbers, grids, grid_size)
}

fn get_winning_sum(grid: HashSet<&u8>, to_exclude: &HashSet<&u8>) -> u32 {
  grid
    .difference(&to_exclude)
    .fold(0, |sum, n| sum + u32::from(**n))
}

/**
 * Given a Vec of guesses and a Vec of grids:
 * - if any grid has a win, return `Some(winning_score)`.
 * - else `None`.
 */
fn get_winning_score(numbers: &[u8], grids: &Vec<Vec<u8>>, grid_size: usize) -> Option<u32> {
  let numbers_set: HashSet<_> = HashSet::from_iter(numbers.iter());

  // TODO: pass this through to calculation w/o mut
  let mut winning_sum: u32 = 0;

  if let Some(_) = grids.iter().find(|grid| {
    for i in 0..(grid_size - 1) {
      let row: HashSet<_> = HashSet::from_iter(&grid[i * grid_size..(i * grid_size + grid_size)]);

      let grid_set: HashSet<_> = HashSet::from_iter(grid.iter());

      if numbers_set.is_superset(&row) {
        winning_sum = get_winning_sum(grid_set, &numbers_set);
        return true;
      }

      let column: HashSet<_> = HashSet::from_iter(
        grid
          .iter()
          .step_by(grid_size)
          .map(|n| n)
          .collect::<Vec<&u8>>(),
      );

      if numbers_set.is_superset(&column) {
        winning_sum = get_winning_sum(grid_set, &numbers_set);
        return true;
      }
    }
    false
  }) {
    return Some(winning_sum);
  } else {
    None
  }
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
  let (numbers, grids, grid_size) = parse_bingo_input(lines);

  // Since a "win" requires at least 5 guesses, eagerly take the first 5.
  let mut guess_index = 4;

  while guess_index < numbers.len() {
    let guesses = &numbers[..guess_index];
    if let Some(winning_score) = get_winning_score(guesses, &grids, grid_size) {
      return winning_score * u32::from(*guesses.last().unwrap());
    }

    guess_index += 1;
  }

  panic!("No winning board found :(");
}
