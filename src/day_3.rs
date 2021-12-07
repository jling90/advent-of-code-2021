use std::fs::File;
use std::io::{self};

// Number of bits for each line in the problem input
const N_BITS: usize = 12;

// For a sequence of size N_BITS strings of 1s and 0s
// Return as a tuple
// - String with 1s in each position where 1 is most common.
// - String with 0s in each position where 1 is most common.
fn most_popular_bit_strings(lines: io::Lines<io::BufReader<File>>) -> (String, String) {
  // Threshold for determining whether '1s' are 'more common'
  // This is half the total number of lines.
  let mut midpoint: f32 = 0.0;
  // Count of '1's in each position
  let mut ones_counts = vec![0; N_BITS];

  let mut gamma = "".to_owned();
  let mut epsilon = "".to_owned();

  for line in lines {
    if let Ok(bits) = line {
      midpoint += 0.5;

      for (index, bit) in bits.char_indices() {
        if bit == '1' {
          ones_counts[index] += 1
        }
      }
    }
  }

  ones_counts.iter().for_each(|n| {
    if *n as f32 >= midpoint {
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
  let (gamma, epsilon) = most_popular_bit_strings(lines);

  return isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();
}
