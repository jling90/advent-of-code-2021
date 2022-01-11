use std::env;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod util;

const EX_1_FILE_PATH: &str = "./resources/day-1.txt";
const EX_2_FILE_PATH: &str = "./resources/day-2.txt";
const EX_3_FILE_PATH: &str = "./resources/day-3.txt";
const EX_4_FILE_PATH: &str = "./resources/day-4.txt";
const EX_5_FILE_PATH: &str = "./resources/day-5.txt";
const EX_6_FILE_PATH: &str = "./resources/day-6.txt";
const EX_7_FILE_PATH: &str = "./resources/day-7.txt";
const EX_8_FILE_PATH: &str = "./resources/day-8.txt";
const EX_9_FILE_PATH: &str = "./resources/day-9.txt";

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
        "2" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_2_FILE_PATH) {
                    println!("{}", day_2::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_2_FILE_PATH) {
                    println!("{}", day_2::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "3" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_3_FILE_PATH) {
                    println!("{}", day_3::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_3_FILE_PATH) {
                    println!("{}", day_3::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "4" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_4_FILE_PATH) {
                    println!("{}", day_4::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_4_FILE_PATH) {
                    println!("{}", day_4::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "5" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_5_FILE_PATH) {
                    println!("{}", day_5::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_5_FILE_PATH) {
                    println!("{}", day_5::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "6" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_6_FILE_PATH) {
                    println!("{}", day_6::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_6_FILE_PATH) {
                    println!("{}", day_6::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "7" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_7_FILE_PATH) {
                    println!("{}", day_7::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_7_FILE_PATH) {
                    println!("{}", day_7::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "8" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_8_FILE_PATH) {
                    println!("{}", day_8::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(EX_8_FILE_PATH) {
                    println!("{}", day_8::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        "9" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(EX_9_FILE_PATH) {
                    println!("{}", day_9::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        _ => println!("Not done with that yet, try another number yo"),
    }
}
