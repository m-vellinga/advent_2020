pub fn run(input: String) {
    println!("Day 2 Part 1: {}", part1(&input));
    println!("Day 2 Part 2: {}", part2(&input));
}

fn part1(input: &String) -> i32 {
    let mut valid_password_count = 0;

    for line in input.lines() {
        let (low, high, character, password) = parse_line(line);

        let amount = password.matches(character).count();
        if amount >= low && amount <= high {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}

fn part2(input: &String) -> i32 {
    let mut valid_password_count = 0;

    for line in input.lines() {
        let (first_position, second_position, character, password) = parse_line(line);
        let positions: Vec<usize> = password.match_indices(character).map(|x| x.0).collect();

        let first_position = first_position - 1;
        let second_position = second_position - 1;

        if positions.contains(&first_position) && positions.contains(&second_position) {
            continue;
        }

        if positions.contains(&first_position) || positions.contains(&second_position) {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}

fn parse_line(line: &str) -> (usize, usize, char, &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 3 {
        panic!("Found invalid format");
    }
    let range: Vec<&str> = parts[0].split('-').collect();
    let range: Vec<usize> = range
        .iter()
        .map(|x| x.trim().parse().expect("Not a valid number"))
        .collect();
    let character = parts[1];
    let character = character.chars().next().unwrap();
    let password = parts[2];

    (range[0], range[1], character, password)
}
