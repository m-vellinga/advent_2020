use regex::RegexSet;

pub fn run(input: String) {
    let passports: Vec<&str> = input.split("\n\n").collect();

    println!("Day 4 Part 1: {}", part_1(&passports));
    println!("Day 4 Part 2: {}", part_2(&passports));
}

fn part_1(passports: &Vec<&str>) -> u64 {
    let set = RegexSet::new(&[
        r"ecl:", r"pid:", r"eyr:", r"hcl:", r"byr:", r"iyr:", r"hgt:",
    ])
    .unwrap();
    return matches_all(passports, set);
}

fn part_2(passports: &Vec<&str>) -> u64 {
    let set = RegexSet::new(&[
        r"\bbyr:(200[0-2]|19[2-9][0-9])\b",
        r"\biyr:20(20|1[0-9])\b",
        r"\beyr:20(30|2[0-9])\b",
        r"\bhgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)\b",
        r"\bhcl:#[0-9a-f]{6}\b",
        r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"\bpid:\d{9}\b",
    ])
    .unwrap();
    return matches_all(passports, set);
}

fn matches_all(passports: &Vec<&str>, set: RegexSet) -> u64 {
    let mut valid_passports: u64 = 0;
    for passport in passports {
        let matches: Vec<_> = set.matches(&passport).into_iter().collect();
        if matches.len() == 7 {
            valid_passports += 1;
        }
    }
    return valid_passports;
}
