use std::cmp::Ordering;
use std::env;
use std::fs;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize = args[1].trim().parse().expect("Not a valid number");

    let day_list = [
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
        day_06::run,
        day_07::run,
        day_08::run,
        day_09::run,
        day_10::run,
    ];

    match day.cmp(&day_list.len()) {
        Ordering::Greater => println!("Day not completed yet!"),
        _ => {
            let input = read_input(&day.to_string());
            let now = Instant::now();
            day_list[day - 1](input);
            println!("{:.2?}", now.elapsed())
        }
    }
}

fn read_input(day: &str) -> String {
    fs::read_to_string(format!("input/day_{:0>2}.txt", day)).expect("Oopsie")
}
