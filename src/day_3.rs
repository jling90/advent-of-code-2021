use std::fs::File;
use std::io::{self};

fn most_popular_bits(lines: &Vec<String>, cmp: &dyn Fn(i32, f64) -> char) -> Vec<char> {
  // Threshold for determining whether '1s' are 'more common'
  // This is half the total number of lines.
  let midpoint = lines.len() as f64 / 2 as f64;
  let input_size = lines[0].len();

  // Count of '1's in each position
  let ones_counts = lines.iter().fold(vec![0; input_size], |mut counts, line| {
    for (index, bit) in line.char_indices() {
      if bit == '1' {
        counts[index] += 1
      }
    }

    counts
  });

  ones_counts.iter().map(|n| cmp(*n, midpoint)).collect()
}

/**
 * Given a vec of Strings, `strings` and a search String `search`,
 * Starting with the minimal substring of `search`, find a set of
 * matching Strings from `strings`. Once a single match exists for
 * a given substring, return it.
 */
fn get_only_substring(strings: &Vec<String>, cmp: &dyn Fn(i32, f64) -> char) -> String {
  let mut strings_copy = strings.clone();
  let input_size = strings[0].len();

  // For each position in `search`, filter out items in
  // `strings_copy` until one remains.
  for cursor in 0..(input_size) {
    let bit_to_check = most_popular_bits(&strings_copy, &cmp)[cursor];

    strings_copy = strings_copy
      .into_iter()
      .filter(|l| l.chars().nth(cursor).unwrap() == bit_to_check)
      .collect();

    if strings_copy.len() == 1 {
      return strings_copy[0].to_string();
    }
  }

  panic!("oh no");
}

// If 1 is most popular, select '1'.
// Break ties by selecting '1'.
fn gamma_cmp(n: i32, midpoint: f64) -> char {
  if n as f64 >= midpoint {
    '1'
  } else {
    '0'
  }
}

// If 1 is most popular, select '0'.
// Break ties by selecting '0'.
fn epsilon_cmp(n: i32, midpoint: f64) -> char {
  if n as f64 >= midpoint {
    '0'
  } else {
    '1'
  }
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> isize {
  let bit_strings = lines.map(|l| l.unwrap()).collect();
  let gamma = String::from_iter(most_popular_bits(&bit_strings, &gamma_cmp));
  let epsilon = String::from_iter(most_popular_bits(&bit_strings, &epsilon_cmp));

  return isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> isize {
  let bit_strings: Vec<String> = lines.map(|l| l.unwrap()).collect();
  let o2_rating = get_only_substring(&bit_strings, &gamma_cmp);
  let co2_rating = get_only_substring(&bit_strings, &epsilon_cmp);

  isize::from_str_radix(&o2_rating, 2).unwrap() * isize::from_str_radix(&co2_rating, 2).unwrap()
}
