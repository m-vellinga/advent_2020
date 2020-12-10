use std::cmp::Ordering;

pub fn run(input: String) {
    let stream = parse_input(input);

    println!("Day 9 Part 1: {}", part_1(&stream, 25));
    println!("Day 9 Part 2: {}", part_2(&stream, 25918798));
}

fn parse_input(input: String) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.trim().parse().expect("Invalid number"))
        .collect()
}

fn part_1(stream: &Vec<i64>, preamble_count: usize) -> u64 {
    let mut preamble = stream[..preamble_count].to_vec();
    let xmas_stream = stream[preamble_count..].to_vec();

    for number in xmas_stream {
        for pre in &preamble {
            let second_number = number - pre;
            if preamble.contains(&second_number) {
                preamble.push(number);
                preamble.remove(0);
                break;
            }
        }
        if preamble.last().unwrap() != &number {
            return number as u64;
        }
    }
    panic!("Not found!");
}

fn part_2(stream: &Vec<i64>, sum_to_find: i64) -> i64 {
    let mut i = 0;
    loop {
        let mut y = i + 1;
        let mut sum = stream[i];
        loop {
            sum += stream[y];
            match sum_to_find.cmp(&sum) {
                Ordering::Less => {
                    i += 1;
                    break;
                }
                Ordering::Greater => y += 1,
                Ordering::Equal => {
                    let mut numbers = stream[i..y].to_vec();
                    numbers.sort();
                    return numbers.first().unwrap() + numbers.last().unwrap();
                }
            }
        }
    }
}

#[test]
fn test_part_1() {
    let input = String::from(
        "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576",
    );

    let stream = parse_input(input);

    let answer = part_1(&stream, 5);
    assert_eq!(answer, 127);
}

#[test]
fn test_part_2() {
    let input = String::from(
        "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576",
    );

    let stream = parse_input(input);

    let answer = part_2(&stream, 127);
    assert_eq!(answer, 62);
}
