use std::collections::HashSet;

pub fn run(input: String) {
    let answers = convert_input(&input);

    println!("Day 6 Part 1: {}", part_1(&answers));
    println!("Day 6 Part 2: {}", part_2(&input));
}

fn convert_input(input: &String) -> Vec<String> {
    let answers: Vec<&str> = input.split("\n\n").collect();
    let answers: Vec<String> = answers
        .iter()
        .map(|x| x.replace("\n", "").replace(" ", ""))
        .collect();
    answers
}

fn part_1(answers: &Vec<String>) -> u64 {
    let mut answered_sum: u64 = 0;
    for group_answer in answers {
        let mut unique_chars = HashSet::new();

        for character in group_answer.chars() {
            unique_chars.insert(character);
        }

        answered_sum += unique_chars.len() as u64
    }
    answered_sum
}

fn part_2(input: &String) -> u64 {
    let group_answers: Vec<&str> = input.split("\n\n").collect();
    let mut group_answered: u64 = 0;

    for group in group_answers {
        let mut first_line_chars = vec![];
        let mut found_in_each_line = HashSet::new();

        for (i, line) in group.lines().enumerate() {
            if i == 0 {
                for character in line.chars() {
                    first_line_chars.push(character);
                    found_in_each_line.insert(character);
                }
            } else {
                for character in &first_line_chars {
                    if !line.contains(*character) {
                        found_in_each_line.remove(character);
                    }
                }
            }
        }
        group_answered += found_in_each_line.len() as u64
    }
    group_answered
}

#[test]
fn example_part_1() {
    let input = String::from(
        "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b",
    );
    let answers = convert_input(&input);

    let sum = part_1(&answers);
    assert_eq!(sum, 11)
}

#[test]
fn example_part_2() {
    let input = String::from(
        "abc

a
b
c

ab
ac

a
a
a
a

b",
    );

    let sum = part_2(&input);
    assert_eq!(sum, 6)
}
