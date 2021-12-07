use std::env;
mod day_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let task_num = &args[2];

    match day_num.as_str() {
        "1" => match task_num.as_str() {
            "1" => day_1::task_one(),
            "2" => day_1::task_two(),
            _ => println!("No match for exercise {}, task {}", day_num, task_num),
        },
        _ => println!("Not done with that yet, try another number yo"),
    }
}
