use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let parsed_data = parse_line(line);
        rules.insert(parsed_data.0, parsed_data.1);
    }

    println!("Day 7 Part 1: {}", part_1(&rules));
    println!("Day 7 Part 2: {}", part_2(&rules));
}

fn parse_line(line: &str) -> (&str, Vec<(u64, &str)>) {
    let line_parts: Vec<&str> = line.trim().split(" bags contain ").collect();

    let bag_name = line_parts.get(0).unwrap();
    let contains_part = line_parts.get(1).unwrap();

    if contains_part == &"no other bags." {
        return (bag_name, vec![]);
    } else {
        let mut contains = vec![];
        let bags: Vec<&str> = contains_part.split(", ").collect();
        for bag_info in bags {
            let bag_info_parts: Vec<&str> = bag_info.split(" bag").collect();
            let bag_info_parts = bag_info_parts.get(0).unwrap().trim();
            let bag_info_parts: Vec<&str> = bag_info_parts.splitn(2, " ").collect();
            contains.push((
                bag_info_parts.get(0).unwrap().parse().expect("Depr"),
                *bag_info_parts.get(1).unwrap(),
            ))
        }
        return (bag_name, contains);
    }
}

fn part_1(rules: &HashMap<&str, Vec<(u64, &str)>>) -> u64 {
    let mut reversed_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for (parent, children) in rules {
        for (_, child) in children {
            let parents = reversed_rules.entry(child).or_insert(vec![]);
            parents.push(parent);
        }
    }

    let mut bags_to_visit = vec!["shiny gold"];
    let mut bag_colors: HashSet<&str> = HashSet::new();

    while !bags_to_visit.is_empty() {
        let bag = bags_to_visit.pop().unwrap();
        if let Some(parents) = reversed_rules.get(bag) {
            for parent in parents {
                if !bag_colors.contains(parent) {
                    bags_to_visit.push(parent);
                    bag_colors.insert(parent);
                }
            }
        }
    }
    bag_colors.len() as u64
}

fn part_2(rules: &HashMap<&str, Vec<(u64, &str)>>) -> u64 {
    recursive_count("shiny gold", rules)
}

fn recursive_count(bag: &str, rules: &HashMap<&str, Vec<(u64, &str)>>) -> u64 {
    if let Some(children) = rules.get(bag) {
        let mut bag_count: u64 = 0;

        for (count, child) in children {
            bag_count += count * recursive_count(child, &rules);
            bag_count += count;
        }
        return bag_count;
    }
    0
}

#[test]
fn test_part_1() {
    let input = String::from(
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.",
    );

    let mut rules = HashMap::new();

    for line in input.lines() {
        let parsed_data = parse_line(line);
        rules.insert(parsed_data.0, parsed_data.1);
    }

    let answer = part_1(&rules);

    assert_eq!(answer, 4);
}

#[test]
fn test_part_2() {
    let input = String::from(
        "shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.",
    );

    let mut rules = HashMap::new();

    for line in input.lines() {
        let parsed_data = parse_line(line);
        rules.insert(parsed_data.0, parsed_data.1);
    }

    let answer = part_2(&rules);

    assert_eq!(answer, 126);
}
