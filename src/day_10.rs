pub fn run(input: String) {
    let jolt_adapters = parse_input(input);

    println!("Day 10 Part 1: {}", part_1(&jolt_adapters));
    println!("Day 10 Part 2: {}", part_2(&jolt_adapters));
}

fn parse_input(input: String) -> Vec<i64> {
    let mut input: Vec<i64> = input
        .lines()
        .map(|l| l.trim().parse().expect("Invalid number"))
        .collect();
    input.sort();
    input
}

fn part_1(jolt_adapters: &Vec<i64>) -> i64 {
    let mut difference_1 = 0;
    // The default device difference is always 3.
    let mut difference_3 = 1;

    let mut previous_jolt = 0;
    for jolts in jolt_adapters {
        match jolts - previous_jolt {
            1 => difference_1 += 1,
            3 => difference_3 += 1,
            _ => (),
        }
        previous_jolt = *jolts;
    }
    difference_1 * difference_3
}

fn part_2(jolt_adapters: &Vec<i64>) -> i64 {
    // All indexes where a single adapter can be removed.
    let mut single_permutation_indexes = vec![];

    let mut previous_jolt = 0;
    for (i, jolt) in jolt_adapters.iter().enumerate() {
        if i + 1 == jolt_adapters.len() {
            break;
        }
        let next_jolt = jolt_adapters[i + 1];
        let difference = next_jolt - previous_jolt;

        if difference <= 3 {
            single_permutation_indexes.push(i);
        }
        previous_jolt = *jolt;
    }

    let mut three_consecutive_permutation_count = 0;
    for (i, permutation_index) in single_permutation_indexes.iter().enumerate() {
        if i + 2 == single_permutation_indexes.len() {
            break;
        }

        if single_permutation_indexes[i + 2] - permutation_index == 2 {
            three_consecutive_permutation_count += 1;
        }
    }

    let single_digit_permutation_count =
        (single_permutation_indexes.len() - (3 * three_consecutive_permutation_count)) as u32;

    // Every 3 consecutive permutations have 7 possible options. Ever single digit permutation has 2.
    return (2 as i64).pow(single_digit_permutation_count)
        * (7 as i64).pow(three_consecutive_permutation_count as u32);
}

#[test]
fn test_part_1() {
    let input = String::from(
        "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3",
    );

    let jolt_adapters = parse_input(input);
    let answer = part_1(&jolt_adapters);

    assert_eq!(answer, 220);
}

#[test]
fn test_part_2() {
    let input = String::from(
        "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3",
    );

    let jolt_adapters = parse_input(input);
    let answer = part_2(&jolt_adapters);

    assert_eq!(answer, 19208);
}
