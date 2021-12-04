
use std::fs;

pub fn task_one() {
  let contents = fs::read_to_string("resources/day-1.txt")
  .expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);
}
