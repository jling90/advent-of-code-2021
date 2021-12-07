use std::env;
mod day_1;
mod util;

const EX_1_FILE_PATH: &str = "./resources/day-1.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let task_num = &args[2];

    match day_num.as_str() {
        "1" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_1_FILE_PATH) {
                    println!("{}", day_1::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_1_FILE_PATH) {
                    println!("{}", day_1::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        _ => println!("Not done with that yet, try another number yo"),
    }
}
