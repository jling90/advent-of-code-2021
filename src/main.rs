use std::env;
mod day_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];

    match day_num.as_str() {
        "1" => day_1::task_one(),
        _ => println!("Not done with that yet, try another number yo"),
    }
}
