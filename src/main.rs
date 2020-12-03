use std::fs;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    day_one::run();
    day_two::run();
    day_three::run(read_input("input/day_three.txt"));
}

fn read_input(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Oopsie")
}
