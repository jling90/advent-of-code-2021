use std::collections::HashSet;
use std::fs::File;
use std::io::{self};

fn input_to_pairs(lines: io::Lines<io::BufReader<File>>) -> Vec<(Vec<String>, Vec<String>)> {
  lines
    .filter_map(|line| line.ok())
    .map(|line| {
      let mut splits = line.split("|");

      let (signal_pattern, outputs) = (splits.next().unwrap(), splits.next().unwrap());

      (
        signal_pattern
          .split_whitespace()
          .map(|s| s.to_string())
          .collect(),
        outputs.split_whitespace().map(|s| s.to_string()).collect(),
      )
    })
    .collect()
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> usize {
  input_to_pairs(lines).iter().fold(0, |sum, (_, outputs)| {
    sum
      + outputs.iter().fold(0, |s, o| {
        s + match o.chars().count() {
          2 | 3 | 4 | 7 => 1,
          _ => 0,
        }
      })
  })
}

/**
 * General strategy for identifying digits:
 * - Some digits (1, 4, 7, 8) are uniquely identifiable by segment count.
 * - For rest, match segments that are shared with these 'known' digits.
 *
 * e.g. 6-segment digits include (0, 6, 9).
 * - only 9 is a superset of 4's segments.
 * - of 0 and 6, only 0 is a superset of 1's segments.
 */
fn get_digits(signal_patterns: &Vec<String>, outputs: &Vec<String>) -> usize {
  let one_segs: HashSet<char> = HashSet::from_iter(
    signal_patterns
      .iter()
      .find(|p| p.chars().count() == 2)
      .unwrap()
      .chars(),
  );

  let four_segs: HashSet<char> = HashSet::from_iter(
    signal_patterns
      .iter()
      .find(|p| p.chars().count() == 4)
      .unwrap()
      .chars(),
  );

  let res = outputs
    .iter()
    .map(|output| match output.chars().count() {
      2 => "1",
      3 => "7",
      4 => "4",
      5 => {
        let output_segs: HashSet<char> = HashSet::from_iter(output.chars());

        if output_segs.is_superset(&one_segs) {
          "3"
        } else if output_segs.union(&four_segs).collect::<Vec<&char>>().len() == 7 {
          "2" // The union of 4 and 2 gives the complete set of segments.
        } else {
          "5"
        }
      }
      6 => {
        let output_segs: HashSet<char> = HashSet::from_iter(output.chars());

        if output_segs.is_superset(&four_segs) {
          "9"
        } else if output_segs.is_superset(&one_segs) {
          "0"
        } else {
          "6"
        }
      }
      7 => "8",
      _ => panic!("invalid sequence"),
    })
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  res
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> usize {
  input_to_pairs(lines)
    .iter()
    .fold(0, |sum, (signal_patterns, outputs)| {
      sum + get_digits(signal_patterns, outputs)
    })
}
