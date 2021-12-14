use std::fs::File;
use std::io::{self};

fn most_popular_bits(lines: &Vec<String>, cmp: &dyn Fn(i32, i32) -> char) -> Vec<char> {
  // Threshold for determining whether '1s' are 'more common'
  // This is half the total number of lines.
  let midpoint = lines.len() / 2;

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

  ones_counts
    .iter()
    .map(|n| cmp(*n, midpoint.try_into().unwrap()))
    .collect()
}

// For a sequence of size N_BITS strings of 1s and 0s
// Return as a tuple
// - String with 1s in each position where 1 is most common.
// - String with 0s in each position where 1 is most common.
fn most_popular_bit_strings(lines: &Vec<String>) -> (String, String) {
  // Threshold for determining whether '1s' are 'more common'
  // This is half the total number of lines.
  let midpoint = lines.len() / 2;

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

  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();

  ones_counts.iter().for_each(|n| {
    if *n >= midpoint {
      gamma.push('1');
      epsilon.push('0');
    } else {
      gamma.push('0');
      epsilon.push('1');
    }
  });

  return (gamma, epsilon);
}

/**
 * Given a vec of Strings, `strings` and a search String `search`,
 * Starting with the minimal substring of `search`, find a set of
 * matching Strings from `strings`. Once a single match exists for
 * a given substring, return it.
 */
fn get_only_substring(strings: &Vec<String>, search: String) -> String {
  let mut strings_copy = strings.clone();
  let bytes = search.as_bytes();
  let input_size = strings_copy[0].len();

  // For each position in `search`, filter out items in
  // `strings_copy` until one remains.
  for cursor in 0..(input_size - 1) {
    strings_copy = strings_copy
      .into_iter()
      .filter(|l| l.as_bytes()[cursor] == bytes[cursor])
      .collect();

    if strings_copy.len() == 1 {
      return strings_copy[0].to_string();
    }

    println!(
      "cursor is {}, strings_copy is {:?}, search is {:?}",
      cursor, strings_copy, search
    );
  }

  // FIXME: Pls no stop why are you doing this to me
  panic!("oh no");
}

fn gamma_cmp(n: i32, midpoint: i32) -> char {
  if n >= midpoint {
    '1'
  } else {
    '0'
  }
}

fn epsilon_cmp(n: i32, midpoint: i32) -> char {
  if n >= midpoint {
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
  let matches: Vec<String> = lines.map(|l| l.unwrap()).collect();
  let (gamma, epsilon) = most_popular_bit_strings(&matches);

  let o2_rating = get_only_substring(&matches, gamma);
  let co2_rating = get_only_substring(&matches, epsilon);

  isize::from_str_radix(&o2_rating, 2).unwrap() * isize::from_str_radix(&co2_rating, 2).unwrap()
}
