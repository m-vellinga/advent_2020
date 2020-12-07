use std::fs;

mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

fn main() {
    day_one::run();
    day_two::run(read_input("two"));
    day_three::run(read_input("three"));
    day_four::run(read_input("four"));
    day_five::run(read_input("five"));
    day_six::run(read_input("six"));
}

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("input/day_{}.txt", day)).expect("Oopsie")
}
