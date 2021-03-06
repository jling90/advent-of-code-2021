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

  // Calculate size of grids by peeking at the first grid row
  let mut grid_iter = lines.peekable();
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

fn get_winning_sum(grid: &[u8], to_exclude: &[u8]) -> u32 {
  let grid_hash: HashSet<&u8> = HashSet::from_iter(grid.iter());
  let exclude_hash: HashSet<&u8> = HashSet::from_iter(to_exclude.iter());

  grid_hash
    .difference(&exclude_hash)
    .fold(0, |sum, n| sum + u32::from(**n))
}

fn check_board_win(numbers: &[u8], grid: &Vec<u8>, grid_size: usize) -> bool {
  let numbers_set: HashSet<_> = HashSet::from_iter(numbers.iter());

  for i in 0..(grid_size - 1) {
    let row: HashSet<_> = HashSet::from_iter(&grid[i * grid_size..(i * grid_size + grid_size)]);
    let column: HashSet<_> = HashSet::from_iter(
      grid
        .iter()
        .step_by(grid_size)
        .map(|n| n)
        .collect::<Vec<&u8>>(),
    );

    if numbers_set.is_superset(&row) || numbers_set.is_superset(&column) {
      return true;
    }
  }

  false
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
  let (numbers, grids, grid_size) = parse_bingo_input(lines);

  // Since a "win" requires at least five guesses,
  // setting initial value to 4 eagerly takes the first five.
  for guess_index in 4..numbers.len() {
    let guesses = &numbers[..guess_index];

    for grid in &grids {
      if check_board_win(guesses, grid, grid_size) {
        return get_winning_sum(grid, guesses) * u32::from(guesses[guess_index - 1]);
      }
    }
  }

  panic!("No winning board found :(");
}

/**
 * Like task_one, but use the last-winning board
 */
pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> u32 {
  let (numbers, mut grids, grid_size) = parse_bingo_input(lines);
  let mut winning_scores: Vec<u32> = vec![];

  // Loop while any non-winning grids remain.
  // When the loop terminates, the score for the last winning
  // board is stored in `final_score`.
  for guess_index in 4..numbers.len() {
    let guesses = &numbers[..guess_index];
    let (won, not_won): (Vec<Vec<u8>>, Vec<Vec<u8>>) = grids
      .into_iter()
      .partition(|grid| check_board_win(guesses, grid, grid_size));

    if not_won.is_empty() {
      break;
    };

    winning_scores.append(
      &mut won
        .iter()
        .map(|winning_grid| {
          get_winning_sum(winning_grid, guesses) * u32::from(guesses[guess_index - 1])
        })
        .collect::<Vec<u32>>(),
    );

    grids = not_won;
  }

  *winning_scores.last().unwrap()
}
