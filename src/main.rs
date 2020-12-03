use std::fs;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    day_one::run();
    day_two::run(read_input("two"));
    day_three::run(read_input("three"));
}

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("input/day_{}.txt", day)).expect("Oopsie")
}
