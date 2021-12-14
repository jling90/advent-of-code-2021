use std::fs::File;
use std::io::{self};

// Number of bits for each line in the problem input
const N_BITS: usize = 12;

// For a sequence of size N_BITS strings of 1s and 0s
// Return as a tuple
// - String with 1s in each position where 1 is most common.
// - String with 0s in each position where 1 is most common.
fn most_popular_bit_strings(lines: Vec<String>) -> (String, String) {
  // Threshold for determining whether '1s' are 'more common'
  // This is half the total number of lines.
  let midpoint = lines.len() / 2;
  // Count of '1's in each position
  let ones_counts = lines.iter().fold(vec![0; N_BITS], |mut counts, line| {
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

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> isize {
  let (gamma, epsilon) = most_popular_bit_strings(lines.map(|l| l.unwrap()).collect());

  return isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();
}
