use std::env;
use std::fs;
use std::time::Instant;

mod day_eight;
mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize = args[1].trim().parse().expect("Not a valid number");

    let now = Instant::now();

    match day {
        1 => day_one::run(),
        2 => day_two::run(read_input("two")),
        3 => day_three::run(read_input("three")),
        4 => day_four::run(read_input("four")),
        5 => day_five::run(read_input("five")),
        6 => day_six::run(read_input("six")),
        7 => day_seven::run(read_input("seven")),
        8 => day_eight::run(read_input("eight")),
        _ => println!("Day not completed yet!"),
    }
    println!("{:.2?}", now.elapsed())
}

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("input/day_{}.txt", day)).expect("Oopsie")
}
