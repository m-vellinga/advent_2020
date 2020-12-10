pub fn run(input: String) {
    let report_input = input
        .lines()
        .map(|l| l.trim().parse().expect("Invalid number"))
        .collect();

    println!("Day 1 Part 1: {}", part_1(&report_input));
    println!("Day 1 Part 2: {}", part_2(&report_input));
}

fn part_1(report_input: &Vec<i32>) -> i32 {
    for first_number in report_input.iter() {
        let second_number = 2020 - first_number;
        if report_input.contains(&second_number) {
            return first_number * second_number;
        }
    }
    panic!("Not found!")
}

fn part_2(report_input: &Vec<i32>) -> i32 {
    for x in 0..report_input.len() {
        for y in 0..report_input.len() {
            for z in 0..report_input.len() {
                let one = report_input[x];
                let two = report_input[y];
                let three = report_input[z];
                if one + two + three == 2020 {
                    return one * two * three;
                }
            }
        }
    }
    panic!("Not found!")
}
